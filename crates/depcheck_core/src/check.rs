use std::collections::{BTreeMap, HashSet};
use std::path::{Component, PathBuf};

use relative_path::RelativePathBuf;
use swc_common::comments::SingleThreadedComments;
use swc_ecma_dep_graph::analyze_dependencies;
use swc_ecma_parser::Syntax;
use walkdir::WalkDir;

use crate::options::CheckerOptions;
use crate::package::{self, Package};
use crate::parsers::Parsers;
use crate::utils::extract_package_name;

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

        let dependencies = self.check_directory(directory, &package);

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
        package: &Package,
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
                    .map(|(module, syntax)| (file, module, syntax))
            })
            .for_each(|(file, module, syntax)| {
                let file_dependencies = analyze_dependencies(&module, &comments);
                let mut file_dependencies: HashSet<_> = file_dependencies
                    .iter()
                    .flat_map(|dependency| {
                        let path = PathBuf::from(&dependency.specifier.to_string());
                        let root = path.components().next()?;

                        if let Component::Normal(_) = root {
                            return extract_package_name(&dependency.specifier);
                        }
                        None
                    })
                    .collect();

                let relative_file_path =
                    RelativePathBuf::from_path(file.path().strip_prefix(&directory).unwrap())
                        .unwrap();

                match syntax {
                    Syntax::Typescript(_) => {
                        let types_dependencies =
                            file_dependencies
                                .clone()
                                .into_iter()
                                .filter_map(|dependency| {
                                    let type_dependency = "@types/".to_owned() + &dependency;
                                    package
                                        .dependencies
                                        .keys()
                                        .find(|&key| !key.starts_with(&type_dependency))
                                        .cloned()
                                });

                        file_dependencies.extend(types_dependencies)
                    }
                    _ => {}
                }
                dependencies.insert(relative_file_path.to_owned(), file_dependencies);
            });
        dependencies
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CheckResult {
    pub package: Package,
    pub using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
}

impl CheckResult {
    pub fn get_missing_dependencies(&self) -> BTreeMap<&str, &HashSet<RelativePathBuf>> {
        let mut package_dependencies = self
            .package
            .dependencies
            .keys()
            .chain(self.package.dev_dependencies.keys());

        let missing = self
            .using_dependencies
            .iter()
            .filter(|(using_dependency, _)| {
                package_dependencies
                    .all(|package_dependency| !using_dependency.starts_with(package_dependency))
            })
            .collect::<Vec<_>>();

        self.using_dependencies
            .iter()
            .filter(|(dependency, _)| {
                !self.package.dependencies.contains_key(*dependency)
                    && !self.package.dev_dependencies.contains_key(*dependency)
            })
            .map(|(dependency, files)| (dependency.as_str(), files))
            .collect()
    }

    pub fn get_unused_dependencies(&self) -> HashSet<&str> {
        let package_dependencies: HashSet<&String> = self.package.dependencies.keys().collect();

        package_dependencies
            .into_iter()
            .filter(|&dependency| !self.using_dependencies.contains_key(dependency))
            .map(|v| v.as_str())
            .collect()
    }

    pub fn get_unused_dev_dependencies(&self) -> HashSet<&str> {
        let package_dev_dependencies: HashSet<&String> =
            self.package.dev_dependencies.keys().collect();

        package_dev_dependencies
            .into_iter()
            .filter(|&dependency| !self.using_dependencies.contains_key(dependency))
            .map(|v| v.as_str())
            .collect()
    }
}
