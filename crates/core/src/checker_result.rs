use crate::config::Config;
use crate::package::{DepsSet, Package};
use crate::util::is_bin_dependency::is_bin_dependency;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct CheckerResult {
    pub package: Package,
    pub directory: PathBuf,
    pub using_dependencies: BTreeMap<String, HashSet<String>>,
    pub config: Config,
}

impl CheckerResult {
    pub fn get_missing_dependencies(&self) -> BTreeMap<&str, &HashSet<String>> {
        if self.config.skip_missing() {
            BTreeMap::new()
        } else {
            let ignore_matches = self
                .config
                .get_ignore_matches()
                .expect("Can't get ignore matches");
            self.using_dependencies
                .iter()
                .filter(|(dependency, _)| !ignore_matches.is_match(dependency.as_str()))
                .filter(|(dependency, _)| !self.package.is_any_dependency(dependency))
                .filter(|(dependency, _)| {
                    !self.config.ignore_bin_package()
                        || !is_bin_dependency(&self.directory, dependency)
                })
                .map(|(dependency, files)| (dependency.as_str(), files))
                .collect()
        }
    }

    pub fn get_unused_dependencies(&self) -> HashSet<&str> {
        self.filter_unused_dependencies(&self.package.dependencies)
    }

    pub fn get_unused_dev_dependencies(&self) -> HashSet<&str> {
        self.filter_unused_dependencies(&self.package.dev_dependencies)
    }

    fn filter_unused_dependencies<'a>(&self, dependencies: &'a DepsSet) -> HashSet<&'a str> {
        let ignore_matches = self
            .config
            .get_ignore_matches()
            .expect("Can't get ignore matches");

        dependencies
            .iter()
            .filter(|(dependency, _)| !ignore_matches.is_match(dependency.as_str()))
            .filter(|(dependency, _)| !self.using_dependencies.contains_key(dependency.as_str()))
            .filter(|(dependency, _)| {
                !self.config.ignore_bin_package() || !is_bin_dependency(&self.directory, dependency)
            })
            .map(|(dependency, _)| dependency.as_str())
            .collect()
    }
}

impl CheckerResult {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn to_pretty_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl Serialize for CheckerResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CheckResult", 4)?;

        let using_dependencies: BTreeMap<&str, HashSet<&str>> = self
            .using_dependencies
            .iter()
            .map(|(dependency, files)| {
                let files = files.iter().map(|path| path.as_str()).collect();
                (dependency.as_str(), files)
            })
            .collect();

        let missing_dependencies: BTreeMap<&str, HashSet<&str>> = self
            .get_missing_dependencies()
            .iter()
            .map(|(&dependency, files)| {
                let files = files.iter().map(|path| path.as_str()).collect();
                (dependency, files)
            })
            .collect();

        state.serialize_field("using_dependencies", &using_dependencies)?;
        state.serialize_field("unused_dependencies", &self.get_unused_dependencies())?;
        state.serialize_field(
            "unused_dev_dependencies",
            &self.get_unused_dev_dependencies(),
        )?;
        state.serialize_field("missing_dependencies", &missing_dependencies)?;

        state.end()
    }
}
