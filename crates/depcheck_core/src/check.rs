use std::collections::{BTreeMap, HashSet};
use std::path::{Component, PathBuf};

use relative_path::RelativePathBuf;
use swc_common::comments::SingleThreadedComments;
use swc_ecma_dep_graph::{analyze_dependencies, DependencyKind};
use swc_ecma_parser::Syntax;
use walkdir::WalkDir;

use crate::options::CheckerOptions;
use crate::package::{self, Package};
use crate::parsers::Parsers;
use crate::util::extract_package_name::extract_package_name;
use crate::util::extract_type_name::extract_type_name;
use crate::util::is_core_module::is_core_module;
use crate::util::is_module::is_module;

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
        let comments = SingleThreadedComments::default();

        WalkDir::new(&directory)
            .into_iter()
            .filter_entry(|entry| {
                let is_root_directory = entry.path() == directory;
                let file_name = entry.file_name().to_string_lossy();
                is_root_directory
                    || (!self.options.ignore_patterns.is_match(file_name.as_ref())
                        && !is_module(entry.path()))
            })
            .filter_map(|entry| Result::ok(entry))
            .filter(|dir_entry| dir_entry.file_type().is_file())
            .filter_map(|file| {
                self.parsers
                    .parse_file(file.path())
                    .map(|(module, syntax)| (file, module, syntax))
            })
            .map(|(file, module, syntax)| {
                let file_dependencies = analyze_dependencies(&module, &comments);
                let file_dependencies: HashSet<_> = file_dependencies
                    .iter()
                    .filter(|dependency| {
                        let path = PathBuf::from(&dependency.specifier.to_string());
                        let root = path.components().next();

                        match root {
                            Some(Component::Normal(_)) => true,
                            _ => false,
                        }
                    })
                    .flat_map(|dependency| {
                        let name = extract_package_name(&dependency.specifier).unwrap();

                        match syntax {
                            Syntax::Typescript(_) => {
                                if dependency.kind == DependencyKind::ImportType {
                                    let type_dependency = "@types/".to_string() + &name;
                                    return if package.dependencies.contains_key(&type_dependency) {
                                        vec![type_dependency]
                                    } else {
                                        vec![]
                                    };
                                }
                                let type_dependency = extract_type_name(&name);
                                if package.dependencies.contains_key(&type_dependency) {
                                    return vec![name, type_dependency];
                                }
                                vec![name]
                            }
                            _ => vec![name],
                        }
                    })
                    .filter(|dependency| !is_core_module(dependency))
                    .collect();

                let relative_file_path =
                    RelativePathBuf::from_path(file.path().strip_prefix(&directory).unwrap())
                        .unwrap();

                (relative_file_path.to_owned(), file_dependencies)
            })
            .collect()
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CheckResult {
    pub package: Package,
    pub using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
}

impl CheckResult {
    pub fn get_missing_dependencies(&self) -> BTreeMap<&str, &HashSet<RelativePathBuf>> {
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
