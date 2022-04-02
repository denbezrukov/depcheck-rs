use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::Path;

/// An ordered map for `dependencies` entries.
pub type DepsSet = BTreeMap<String, String>;
/// An ordered map for `bin` entries.
pub type BinSet = BTreeMap<String, String>;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// The package name.
    #[serde(default)]
    pub name: String,
    /// The package version.
    #[serde(default)]
    pub version: String,
    /// The optional list of dependencies.
    #[serde(default)]
    pub dependencies: DepsSet,
    /// The optional list of development dependencies.
    #[serde(default)]
    pub dev_dependencies: DepsSet,
    /// The optional list of peer dependencies.
    #[serde(default)]
    pub peer_dependencies: DepsSet,
    /// The optional list of bundled dependencies.
    #[serde(default)]
    pub bundled_dependencies: DepsSet,
    /// The optional list of optional dependencies.
    #[serde(default)]
    pub optional_dependencies: DepsSet,
    /// The optional set of binary definitions.
    #[serde(default)]
    pub bin: Option<BinSet>,
}

impl Package {
    /// Creates a new default package.
    pub fn new() -> Self {
        Self::default()
    }

    /// Deserializes a `Package` from a file path.
    pub fn from_path<T: AsRef<Path>>(path: T) -> eyre::Result<Self> {
        let content = fs::read(path.as_ref())?;
        Self::from_slice(content.as_slice())
    }

    /// Deserializes a `Package` from an IO stream.
    pub fn from_reader<R: Read>(r: R) -> eyre::Result<Self> {
        Ok(serde_json::from_reader(r)?)
    }

    /// Deserializes a `Package` from bytes.
    pub fn from_slice(v: &[u8]) -> eyre::Result<Self> {
        Ok(serde_json::from_slice(v)?)
    }
}

impl Package {
    pub fn is_dependency(&self, dependency: &str) -> bool {
        self.dependencies.contains_key(dependency)
    }

    pub fn is_dev_dependency(&self, dependency: &str) -> bool {
        self.dev_dependencies.contains_key(dependency)
    }

    pub fn is_peer_dependency(&self, dependency: &str) -> bool {
        self.peer_dependencies.contains_key(dependency)
    }

    pub fn is_optional_dependency(&self, dependency: &str) -> bool {
        self.optional_dependencies.contains_key(dependency)
    }

    pub fn is_any_dependency(&self, dependency: &str) -> bool {
        self.is_dependency(dependency)
            || self.is_dev_dependency(dependency)
            || self.is_peer_dependency(dependency)
            || self.is_optional_dependency(dependency)
    }
}
