// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

//! NIPC0: Code Snippets
//!
//! <https://github.com/nostr-protocol/nips/blob/master/C0.md>

use alloc::string::String;
use alloc::vec::Vec;

use crate::{EventBuilder, Kind, TagStandard};

/// Code snippet
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeSnippet {
    /// The code snippet.
    pub snippet: String,
    /// Programming language name.
    /// Examples: "javascript", "python", "rust"
    pub language: Option<String>,
    /// Name of the code snippet, commonly a filename.
    /// Examples: "hello-world.js", "quick-sort.py"
    pub name: Option<String>,
    /// File extension (without the dot).
    /// Examples: "js", "py", "rs"
    pub extension: Option<String>,
    /// Brief description of what the code does
    pub description: Option<String>,
    /// Runtime or environment specification.
    /// Example: "node v18.15.0", "python 3.11"
    pub runtime: Option<String>,
    /// License under which the code is shared.
    /// Example: "MIT", "GPL-3.0", "Apache-2.0"
    pub license: Option<String>,
    /// Dependencies required for the code to run.
    pub dependencies: Vec<String>,
    /// Reference to a repository where this code originates.
    pub repo: Option<String>,
}

impl From<CodeSnippet> for EventBuilder {
    /// Convert the code snippet to an event builder
    fn from(snippet: CodeSnippet) -> Self {
        let mut tags = Vec::new();

        let mut add_if_some = |tag: Option<TagStandard>| {
            if let Some(tag) = tag {
                tags.push(tag.into());
            }
        };

        // `l` tag used for label in all event kinds except Code Snippets (1337)
        // is used as the programming language
        add_if_some(snippet.language.map(|l| TagStandard::Label(vec![l])));
        add_if_some(snippet.name.map(TagStandard::Name));
        add_if_some(snippet.extension.map(TagStandard::Extension));
        add_if_some(snippet.description.map(TagStandard::Description));
        add_if_some(snippet.runtime.map(TagStandard::Runtime));
        add_if_some(snippet.license.map(TagStandard::License));
        add_if_some(snippet.repo.map(TagStandard::Repository));
        for dep in snippet.dependencies {
            tags.push(TagStandard::Dependency(dep).into());
        }

        EventBuilder::new(Kind::CodeSnippet, snippet.snippet).tags(tags)
    }
}

impl CodeSnippet {
    /// Create a new code snippet
    #[inline]
    pub fn new(snippet: impl Into<String>) -> Self {
        Self {
            snippet: snippet.into(),
            ..Default::default()
        }
    }

    /// Set the programming language name. e.g. "javascript", "python", "rust"
    pub fn language(mut self, lang: impl AsRef<str>) -> Self {
        self.language = Some(lang.as_ref().to_lowercase());
        self
    }

    /// Set the name of the code snippet, commonly a filename.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the file extension (without the dot).
    pub fn extension(mut self, extension: impl Into<String>) -> Self {
        self.extension = Some(extension.into());
        self
    }

    /// Set a brief description of what the code does
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the runtime or environment specification. e.g. "node v18.15.0", "python 3.11"
    pub fn runtime(mut self, runtime: impl Into<String>) -> Self {
        self.runtime = Some(runtime.into());
        self
    }

    /// Set the license under which the code is shared. e.g. "MIT", "GPL-3.0", "Apache-2.0"
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Add a dependency required for the code to run.
    pub fn dependencies(mut self, dep: impl Into<String>) -> Self {
        let dep = dep.into();
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
        }
        self
    }

    /// Set the repository where this code originates.
    pub fn repo(mut self, repo: impl Into<String>) -> Self {
        self.repo = Some(repo.into());
        self
    }
}
