use std::collections::{BTreeMap, HashSet};
use std::path::{Component, PathBuf};

use relative_path::{RelativePath, RelativePathBuf};
use swc_common::comments::SingleThreadedComments;
use swc_ecma_dep_graph::analyze_dependencies;
use walkdir::WalkDir;

use crate::options::CheckerOptions;
use crate::package::{self, Package};
use crate::parsers::Parsers;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CheckResult {
    pub package: Package,
    pub using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
}

impl CheckResult {
    pub fn get_result(&self) -> CheckDerive {
        let missing_dependencies: BTreeMap<_, _> = self
            .using_dependencies
            .iter()
            .filter(|(dependency, _)| {
                !self.package.dependencies.contains_key(*dependency)
                    && !self.package.dev_dependencies.contains_key(*dependency)
            })
            .map(|(dependency, files)| (dependency.as_str(), files))
            .collect();

        let package_dependencies: HashSet<&String> = self.package.dependencies.keys().collect();
        let package_dev_dependencies: HashSet<&String> =
            self.package.dev_dependencies.keys().collect();
        let exclusive_using_dependencies = self.using_dependencies.keys().collect();

        let unused_dependencies: HashSet<_> = package_dependencies
            .difference(&exclusive_using_dependencies)
            .map(|v| v.as_str())
            .collect();

        let unused_dev_dependencies: HashSet<_> = package_dev_dependencies
            .difference(&exclusive_using_dependencies)
            .map(|v| v.as_str())
            .collect();

        CheckDerive {
            unused_dependencies,
            unused_dev_dependencies,
            missing_dependencies,
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct CheckDerive<'a> {
    pub unused_dependencies: HashSet<&'a str>,
    pub unused_dev_dependencies: HashSet<&'a str>,
    pub missing_dependencies: BTreeMap<&'a str, &'a HashSet<RelativePathBuf>>,
}

pub struct Checker {
    options: CheckerOptions,
    parsers: Parsers,
}

impl Default for Checker {
    fn default() -> Self {
        Checker {
            options: Default::default(),
            parsers: Default::default(),
        }
    }
}

impl Checker {
    pub fn check_package(&self, directory: PathBuf) -> package::Result<CheckResult> {
        let mut package_path = directory.to_owned();
        package_path.push("package.json");

        let package = Package::from_path(package_path)?;

        let dependencies = self.check_directory(directory);

        let mut using_dependencies = BTreeMap::new();

        dependencies.into_iter().for_each(|(path, dependencies)| {
            dependencies.iter().for_each(|dependency| {
                let files = using_dependencies
                    .entry(dependency.clone())
                    .or_insert(HashSet::with_capacity(100));
                files.insert(path.clone());
            })
        });

        Ok(CheckResult {
            package,
            using_dependencies,
        })
    }

    pub fn check_directory(
        &self,
        directory: PathBuf,
    ) -> BTreeMap<RelativePathBuf, HashSet<String>> {
        let mut dependencies = BTreeMap::new();
        let comments = SingleThreadedComments::default();

        WalkDir::new(&directory)
            .into_iter()
            .filter_entry(|entry| {
                let file_name = entry.file_name().to_string_lossy();
                !self.options.ignore_patterns.is_match(file_name.as_ref())
            })
            .filter_map(|entry| Result::ok(entry))
            .filter(|dir_entry| dir_entry.file_type().is_file())
            .filter_map(|file| {
                self.parsers
                    .parse_file(file.path())
                    .map(|module| (file, module))
            })
            .for_each(|(file, module)| {
                let file_dependencies = analyze_dependencies(&module, &comments);
                let file_dependencies: HashSet<_> = file_dependencies
                    .iter()
                    .flat_map(|dependency| {
                        let dependency = PathBuf::from(dependency.specifier.to_string());
                        let root = dependency.components().next()?;
                        match root {
                            Component::Normal(root) => Some(root.to_str()?.to_string()),
                            _ => None,
                        }
                    })
                    .collect();

                let relative_file_path =
                    RelativePathBuf::from_path(file.path().strip_prefix(&directory).unwrap())
                        .unwrap();
                dependencies.insert(relative_file_path.to_owned(), file_dependencies);
            });
        dependencies
    }
}
