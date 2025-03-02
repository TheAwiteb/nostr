// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;

use nostr::util::BoxedFuture;
use nostr::{Event, EventId};

/// Negentropy sync [`Event`] uploader
pub trait SyncUploader: fmt::Debug + Send + Sync {
    /// Get [`Event`] by [`EventId`] for sync event uploading
    ///
    /// Get the event from an in-memory or persistent database to allow the uploading of events during the "UP" stage.
    fn get_event_by_id<'a>(&'a self, id: &'a EventId) -> BoxedFuture<'a, Option<Event>>;
}
