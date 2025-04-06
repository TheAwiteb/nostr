// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use nostr_sdk::prelude::*;

const EXAMPLE_SNIPPET: &str = include_str!("code_snippet.rs");

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let keys = Keys::generate();
    let client = Client::new(keys);

    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nos.lol").await?;
    client.add_relay("wss://nostr.mom").await?;

    client.connect().await;

    // Build a code snippet for this example :)
    let snippet = EventBuilder::code_snippet(
        CodeSnippet::new(EXAMPLE_SNIPPET)
            .name("code_snippts.rs")
            .description("Snippet that snippet itself")
            .language("rust")
            .extension("rs")
            .license("MIT"),
    );

    let event = client.send_event_builder(snippet).await.unwrap();
    let nevent = Nip19Event::new(*event.id()).relays(vec![
        RelayUrl::parse("wss://nos.lol").unwrap(),
        RelayUrl::parse("wss://nostr.mom").unwrap(),
    ]);

    tracing::info!("Done, check the event `{}`", nevent.to_bech32().unwrap());

    client.shutdown().await;

    Ok(())
}
