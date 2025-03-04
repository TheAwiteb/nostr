use std::sync::Arc;

use nostr::prelude::BoxedFuture;
use nostr::{EventBuilder, NostrSigner, RelayUrl};
use nostr_relay_pool::middleware::{AuthenticationLayer, AuthenticationStatus, MiddlewareError};
use tokio::sync::RwLock;

use crate::client::Error;

#[derive(Debug)]
pub(super) struct AuthenticationMiddleware {
    pub signer: Arc<RwLock<Option<Arc<dyn NostrSigner>>>>,
}

impl AuthenticationLayer for AuthenticationMiddleware {
    fn build_authentication<'a>(
        &'a self,
        relay_url: &'a RelayUrl,
        challenge: &'a str,
    ) -> BoxedFuture<'a, Result<AuthenticationStatus, MiddlewareError>> {
        Box::pin(async move {
            // Acquire signer lock
            let signer = self.signer.read().await;

            // Get signer
            let signer = signer
                .as_ref()
                .ok_or(MiddlewareError::backend(Error::SignerNotConfigured))?;

            // Construct event
            let event = EventBuilder::auth(challenge, relay_url.clone())
                .sign(signer)
                .await
                .map_err(MiddlewareError::backend)?;

            // Return
            Ok(AuthenticationStatus::Success(event))
        })
    }
}

// TODO: add unit test for NIP42 REQ and EVENT in the SDK
