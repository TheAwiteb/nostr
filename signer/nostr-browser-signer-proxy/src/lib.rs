// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

//! Proxy to use Nostr Browser signer ([NIP-07](https://github.com/nostr-protocol/nips/blob/master/07.md)) in native applications.
//!
//! <https://github.com/nostr-protocol/nips/blob/master/07.md>

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::large_futures)]
#![warn(rustdoc::bare_urls)]

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use nostr::prelude::{BoxedFuture, SignerBackend};
use nostr::{Event, NostrSigner, PublicKey, SignerError, UnsignedEvent};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{oneshot, Mutex, Notify, OnceCell};
use tokio::task::JoinHandle;
use tokio::time;
use uuid::Uuid;

mod error;
pub mod prelude;

pub use self::error::Error;

const HTML: &str = include_str!("../index.html");
const JS: &str = include_str!("../proxy.js");
const CSS: &str = include_str!("../style.css");
const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
const TIMEOUT: Duration = Duration::from_secs(30);

type PendingResponseMap = HashMap<Uuid, oneshot::Sender<Result<Value, String>>>;

#[derive(Debug, Deserialize)]
struct Message {
    id: Uuid,
    error: Option<String>,
    result: Option<Value>,
}

#[derive(Debug, Clone, Copy)]
enum RequestMethod {
    GetPublicKey,
    SignEvent,
    Nip04Encrypt,
    Nip04Decrypt,
    Nip44Encrypt,
    Nip44Decrypt,
}

impl RequestMethod {
    fn as_str(&self) -> &str {
        match self {
            Self::GetPublicKey => "get_public_key",
            Self::SignEvent => "sign_event",
            Self::Nip04Encrypt => "nip04_encrypt",
            Self::Nip04Decrypt => "nip04_decrypt",
            Self::Nip44Encrypt => "nip44_encrypt",
            Self::Nip44Decrypt => "nip44_decrypt",
        }
    }
}

impl Serialize for RequestMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize)]
struct RequestData {
    id: Uuid,
    method: RequestMethod,
    params: Value,
}

#[derive(Serialize)]
struct Requests<'a> {
    requests: &'a [RequestData],
}

/// Params for NIP-04 and NIP-44 encryption/decryption
#[derive(Serialize)]
struct CryptoParams<'a> {
    public_key: &'a PublicKey,
    content: &'a str,
}

#[derive(Debug, Clone)]
struct ProxyState {
    // Requests waiting to be picked up by browser
    pub outgoing_requests: Arc<Mutex<Vec<RequestData>>>,
    // Map of request ID to response sender
    pub pending_responses: Arc<Mutex<PendingResponseMap>>,
}

/// Nostr Browser Signer Proxy
///
/// Proxy to use Nostr Browser signer (NIP-07) in native applications.
#[derive(Debug, Clone)]
pub struct BrowserSignerProxy {
    port: u16,
    state: ProxyState,
    handle: OnceCell<Arc<JoinHandle<()>>>,
    shutdown: Arc<Notify>,
}

// TODO: use atomic-destructor to automatically shutdown this when all instances are dropped

impl BrowserSignerProxy {
    // TODO: use a builder instead, to allow to config IP, port, timeout and so on.
    /// Construct a new browser signer proxy
    pub fn new(port: u16) -> Self {
        let state = ProxyState {
            outgoing_requests: Arc::new(Mutex::new(Vec::new())),
            pending_responses: Arc::new(Mutex::new(HashMap::new())),
        };

        Self {
            port,
            state,
            handle: OnceCell::new(),
            shutdown: Arc::new(Notify::new()),
        }
    }

    /// Get the signer proxy webpage URL
    #[inline]
    pub fn url(&self) -> String {
        format!("http://{IP_ADDR}:{}", self.port)
    }

    /// Start the proxy
    ///
    /// If this is not called, will be automatically started on the first interaction with the signer.
    pub async fn start(&self) -> Result<(), Error> {
        let _handle: &Arc<JoinHandle<()>> = self
            .handle
            .get_or_try_init(|| async {
                let addr: SocketAddr = SocketAddr::new(IP_ADDR, self.port);
                let listener = TcpListener::bind(addr).await?;

                let state = self.state.clone();
                let shutdown = self.shutdown.clone();

                let handle: JoinHandle<()> = tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            res = listener.accept() => {
                                let stream: TcpStream = match res {
                                    Ok((stream, ..)) => stream,
                                    Err(e) => {
                                        tracing::error!("Failed to accept connection: {}", e);
                                        continue;
                                    }
                                };

                                let io: TokioIo<TcpStream> = TokioIo::new(stream);
                                let state: ProxyState = state.clone();

                                tokio::spawn(async move {
                                    let service = service_fn(move |req| {
                                        handle_request(req, state.clone())
                                    });

                                    if let Err(e) = http1::Builder::new()
                                        .serve_connection(io, service)
                                        .await
                                    {
                                        tracing::error!("Error serving connection: {e}");
                                    }
                                });
                            },
                            _ = shutdown.notified() => {
                                break;
                            }
                        }
                    }

                    tracing::info!("Shutting down proxy server.");
                });

                Ok::<_, Error>(Arc::new(handle))
            })
            .await?;
        Ok(())
    }

    async fn request<T>(&self, method: RequestMethod, params: Value) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.start().await?;

        let request_id: Uuid = Uuid::new_v4();
        let (tx, rx) = oneshot::channel();

        // Store the response sender
        {
            let mut pending_responses = self.state.pending_responses.lock().await;
            pending_responses.insert(request_id, tx);
        }

        // Add to outgoing requests queue
        {
            let mut outgoing_requests = self.state.outgoing_requests.lock().await;
            outgoing_requests.push(RequestData {
                id: request_id,
                method,
                params,
            });
        }

        // Wait for response
        match time::timeout(TIMEOUT, rx)
            .await
            .map_err(|_| Error::Timeout)??
        {
            Ok(res) => Ok(serde_json::from_value(res)?),
            Err(error) => Err(Error::Generic(error)),
        }
    }

    #[inline]
    async fn _get_public_key(&self) -> Result<PublicKey, Error> {
        self.request(RequestMethod::GetPublicKey, json!({})).await
    }

    #[inline]
    async fn _sign_event(&self, event: UnsignedEvent) -> Result<Event, Error> {
        let event: Event = self
            .request(RequestMethod::SignEvent, serde_json::to_value(event)?)
            .await?;
        event.verify()?;
        Ok(event)
    }

    #[inline]
    async fn _nip04_encrypt(&self, public_key: &PublicKey, content: &str) -> Result<String, Error> {
        let params = CryptoParams {
            public_key,
            content,
        };
        self.request(RequestMethod::Nip04Encrypt, serde_json::to_value(params)?)
            .await
    }

    #[inline]
    async fn _nip04_decrypt(&self, public_key: &PublicKey, content: &str) -> Result<String, Error> {
        let params = CryptoParams {
            public_key,
            content,
        };
        self.request(RequestMethod::Nip04Decrypt, serde_json::to_value(params)?)
            .await
    }

    #[inline]
    async fn _nip44_encrypt(&self, public_key: &PublicKey, content: &str) -> Result<String, Error> {
        let params = CryptoParams {
            public_key,
            content,
        };
        self.request(RequestMethod::Nip44Encrypt, serde_json::to_value(params)?)
            .await
    }

    #[inline]
    async fn _nip44_decrypt(&self, public_key: &PublicKey, content: &str) -> Result<String, Error> {
        let params = CryptoParams {
            public_key,
            content,
        };
        self.request(RequestMethod::Nip44Decrypt, serde_json::to_value(params)?)
            .await
    }
}

impl NostrSigner for BrowserSignerProxy {
    fn backend(&self) -> SignerBackend {
        SignerBackend::BrowserExtension
    }

    #[inline]
    fn get_public_key(&self) -> BoxedFuture<Result<PublicKey, SignerError>> {
        Box::pin(async move { self._get_public_key().await.map_err(SignerError::backend) })
    }

    #[inline]
    fn sign_event(&self, unsigned: UnsignedEvent) -> BoxedFuture<Result<Event, SignerError>> {
        Box::pin(async move {
            self._sign_event(unsigned)
                .await
                .map_err(SignerError::backend)
        })
    }

    #[inline]
    fn nip04_encrypt<'a>(
        &'a self,
        public_key: &'a PublicKey,
        content: &'a str,
    ) -> BoxedFuture<'a, Result<String, SignerError>> {
        Box::pin(async move {
            self._nip04_encrypt(public_key, content)
                .await
                .map_err(SignerError::backend)
        })
    }

    #[inline]
    fn nip04_decrypt<'a>(
        &'a self,
        public_key: &'a PublicKey,
        encrypted_content: &'a str,
    ) -> BoxedFuture<'a, Result<String, SignerError>> {
        Box::pin(async move {
            self._nip04_decrypt(public_key, encrypted_content)
                .await
                .map_err(SignerError::backend)
        })
    }

    #[inline]
    fn nip44_encrypt<'a>(
        &'a self,
        public_key: &'a PublicKey,
        content: &'a str,
    ) -> BoxedFuture<'a, Result<String, SignerError>> {
        Box::pin(async move {
            self._nip44_encrypt(public_key, content)
                .await
                .map_err(SignerError::backend)
        })
    }

    #[inline]
    fn nip44_decrypt<'a>(
        &'a self,
        public_key: &'a PublicKey,
        payload: &'a str,
    ) -> BoxedFuture<'a, Result<String, SignerError>> {
        Box::pin(async move {
            self._nip44_decrypt(public_key, payload)
                .await
                .map_err(SignerError::backend)
        })
    }
}

async fn handle_request(
    req: Request<Incoming>,
    state: ProxyState,
) -> Result<Response<BoxBody<Bytes, Error>>, Error> {
    match (req.method(), req.uri().path()) {
        // Serve the HTML proxy page
        (&Method::GET, "/") => Ok(Response::builder()
            .header("Content-Type", "text/html")
            .body(full(HTML))?),
        // Serve the CSS page style
        (&Method::GET, "/style.css") => Ok(Response::builder()
            .header("Content-Type", "text/css")
            .body(full(CSS))?),
        // Serve the JS proxy script
        (&Method::GET, "/proxy.js") => Ok(Response::builder()
            .header("Content-Type", "application/javascript")
            .body(full(JS))?),
        // Browser polls this endpoint to get pending requests
        (&Method::GET, "/api/pending") => {
            let mut outgoing = state.outgoing_requests.lock().await;

            let data: Requests<'_> = Requests {
                requests: &outgoing,
            };
            let json: String = serde_json::to_string(&data)?;

            tracing::debug!(
                "Sending {} pending requests to browser",
                data.requests.len()
            );

            // Clear the outgoing requests after sending them
            outgoing.clear();

            Ok(Response::builder()
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(full(json))?)
        }
        // Get response
        (&Method::POST, "/api/response") => {
            // Correctly collect the body bytes from the stream
            let body_bytes: Bytes = match req.into_body().collect().await {
                Ok(collected) => collected.to_bytes(),
                Err(e) => {
                    tracing::error!("Failed to read body: {e}");
                    let response = Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(full("Failed to read body"))?;
                    return Ok(response);
                }
            };

            // Handle responses from the browser extension
            let message: Message = match serde_json::from_slice(&body_bytes) {
                Ok(json) => json,
                Err(_) => {
                    let response = Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(full("Invalid JSON"))?;
                    return Ok(response);
                }
            };

            tracing::debug!("Received response from browser: {:?}", message);

            let mut pending = state.pending_responses.lock().await;

            if let Some(sender) = pending.remove(&message.id) {
                let result: Result<Value, String> = if let Some(error) = message.error {
                    Err(error)
                } else {
                    Ok(message.result.unwrap_or(Value::Null))
                };

                let _ = sender.send(result);

                tracing::info!("Forwarded response for request {}", message.id);
            } else {
                tracing::warn!("No pending request found for {}", message.id);
            }

            let response = Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .body(full("OK"))?;
            Ok(response)
        }
        (&Method::OPTIONS, _) => {
            // Handle CORS preflight requests
            let response = Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body(full(""))?;
            Ok(response)
        }
        // 404 - not found
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full("Not Found"))?;
            Ok(response)
        }
    }
}

#[inline]
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
