use crate::config::Config;
use crate::package::{DepsSet, Package};
use crate::util::is_bin_dependency::is_bin_dependency;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

/// Dependencies checker result.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckerResult {
    /// Using dependencies in directory. Key is dependency and value is unique file paths.
    pub using_dependencies: BTreeMap<String, HashSet<String>>,
    /// Missing dependencies in directory. Key is dependency and value is unique file paths.
    pub missing_dependencies: BTreeMap<String, HashSet<String>>,
    /// Unique unused dependencies.
    pub unused_dependencies: HashSet<String>,
    /// Unique unused dev dependencies.
    pub unused_dev_dependencies: HashSet<String>,
}

impl CheckerResult {
    pub fn new(
        using_dependencies: BTreeMap<String, HashSet<String>>,
        package: Package,
        config: &Config,
    ) -> CheckerResult {
        let ignore_matches = config
            .get_ignore_matches()
            .expect("Can't get ignore matches");

        let missing_dependencies = if config.skip_missing() {
            BTreeMap::new()
        } else {
            using_dependencies
                .iter()
                .filter(|(dependency, _)| !ignore_matches.is_match(dependency.as_str()))
                .filter(|(dependency, _)| !package.is_any_dependency(dependency))
                .filter(|(dependency, _)| {
                    !config.ignore_bin_package()
                        || !is_bin_dependency(config.get_directory(), dependency)
                })
                .map(|(dependency, files)| {
                    (
                        dependency.to_owned(),
                        files.iter().map(|file| file.to_owned()).collect(),
                    )
                })
                .collect()
        };

        let Package {
            dependencies,
            dev_dependencies,
            ..
        } = package;

        let filter_dependencies = |deps: DepsSet| {
            deps.into_iter()
                .filter(|(dependency, _)| !ignore_matches.is_match(dependency.as_str()))
                .filter(|(dependency, _)| !using_dependencies.contains_key(dependency.as_str()))
                .filter(|(dependency, _)| {
                    !config.ignore_bin_package()
                        || !is_bin_dependency(config.get_directory(), dependency)
                })
                .map(|(dependency, _)| dependency)
                .collect()
        };

        let unused_dependencies = filter_dependencies(dependencies);
        let unused_dev_dependencies = filter_dependencies(dev_dependencies);

        CheckerResult {
            using_dependencies,
            missing_dependencies,
            unused_dependencies,
            unused_dev_dependencies,
        }
    }
}
