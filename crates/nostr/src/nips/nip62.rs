// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

//! NIP-62: Request to Vanish
//!
//! https://github.com/nostr-protocol/nips/blob/master/62.md

use crate::{RelayUrl, Tag, TagStandard, Tags};

/// Request to Vanish target, which is multiple relays or all relays "ALL_RELAYS".
pub enum VanishTarget {
    /// All relays target. `["r", "ALL_RELAYS"]`
    AllRelays,
    /// Multiple relay targets. `[["r", "RELAY"], ...]`
    Relays(Vec<RelayUrl>),
}

impl VanishTarget {
    /// A single relay target.
    pub fn relay(relay: RelayUrl) -> Self {
        Self::Relays(vec![relay])
    }

    /// Multiple relay targets. If the given iterator is empty, the target is all relays.
    pub fn relays<I>(relays: I) -> Self
    where
        I: IntoIterator<Item = RelayUrl>,
    {
        let relays: Vec<_> = relays.into_iter().collect();
        if relays.is_empty() {
            Self::AllRelays
        } else {
            Self::Relays(relays)
        }
    }

    /// All relays target. This is equivalent to `VanishTarget::relays(vec![])`.
    pub fn all_relays() -> Self {
        Self::AllRelays
    }
}

impl From<VanishTarget> for Vec<TagStandard> {
    fn from(vanish_target: VanishTarget) -> Self {
        match vanish_target {
            VanishTarget::AllRelays => vec![TagStandard::AllRelays],
            VanishTarget::Relays(relays) => relays.into_iter().map(TagStandard::Relay).collect(),
        }
    }
}

impl From<VanishTarget> for Tags {
    fn from(vanish_target: VanishTarget) -> Self {
        Self::from_list(
            Vec::<TagStandard>::from(vanish_target)
                .into_iter()
                .map(Tag::from)
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::VanishTarget;
    use crate::Tags;

    #[test]
    fn test_single_relay() {
        let expected_tag = ["relay", "wss://example.com"];
        let tag = VanishTarget::relay("wss://example.com".parse().unwrap());
        let tags = Tags::from(tag);
        assert_eq!(tags.len(), 1);

        assert_eq!(tags.first().unwrap().as_slice(), expected_tag)
    }

    #[test]
    fn test_relays() {
        let expected_tag = vec![
            ["relay", "wss://example.com"],
            ["relay", "wss://example1.com"],
        ];
        let tag = VanishTarget::relays([
            "wss://example.com".parse().unwrap(),
            "wss://example1.com".parse().unwrap(),
        ]);
        let tags = Tags::from(tag);
        assert_eq!(tags.len(), 2);

        assert_eq!(
            tags.to_vec()
                .into_iter()
                .map(|t| t.as_slice().to_vec())
                .collect::<Vec<_>>(),
            expected_tag
        )
    }

    #[test]
    fn test_all_relays() {
        let expected_tag = ["relay", "ALL_RELAYS"];
        let tag = VanishTarget::all_relays();
        let empty_relays = VanishTarget::relays(Vec::new());
        let tags = Tags::from(tag);
        assert_eq!(tags.len(), 1);

        assert_eq!(tags.first().unwrap().as_slice(), expected_tag);
        assert_eq!(
            Tags::from(empty_relays).first().unwrap().as_slice(),
            expected_tag
        );
    }
}
