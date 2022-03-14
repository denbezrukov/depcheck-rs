use std::collections::{BTreeMap, HashSet};
use std::path::{Component, Path, PathBuf};

use relative_path::RelativePathBuf;
use swc_common::comments::SingleThreadedComments;
use swc_ecma_dep_graph::{analyze_dependencies, DependencyKind};
use swc_ecma_parser::Syntax;
use walkdir::WalkDir;

use crate::options::CheckerOptions;
use crate::package::{self, Package};
use crate::parser::Parser;
use crate::util::extract_package_name::extract_package_name;
use crate::util::extract_type_name::extract_type_name;
use crate::util::is_bin_dependency::is_bin_dependency;
use crate::util::is_core_module::is_core_module;
use crate::util::is_module::is_module;
use crate::util::load_module::load_module;
use std::iter;

#[derive(Default)]
pub struct Checker {
    options: CheckerOptions,
    parsers: Parser,
}

impl Checker {
    pub fn new(options: CheckerOptions) -> Self {
        Checker {
            options,
            ..Default::default()
        }
    }
}

impl Checker {
    pub fn check_package(&self, directory: PathBuf) -> package::Result<CheckResult> {
        let package = load_module(&directory)?;

        let dependencies = self.check_directory(&directory, &package);

        let mut using_dependencies = BTreeMap::new();

        dependencies.into_iter().for_each(|(path, dependencies)| {
            dependencies.iter().for_each(|dependency| {
                let files = using_dependencies
                    .entry(dependency.clone())
                    .or_insert_with(|| HashSet::with_capacity(100));
                files.insert(path.clone());
            })
        });

        Ok(CheckResult {
            package,
            directory,
            using_dependencies,
            options: self.options.clone(),
        })
    }

    pub fn check_directory(
        &self,
        directory: &Path,
        package: &Package,
    ) -> BTreeMap<RelativePathBuf, HashSet<String>> {
        let comments = SingleThreadedComments::default();

        WalkDir::new(directory)
            .into_iter()
            .filter_entry(|entry| {
                let is_root_directory = entry.path() == directory;
                let file_name = entry.file_name().to_string_lossy();
                is_root_directory
                    || (!self
                        .options
                        .get_ignore_patterns()
                        .is_match(file_name.as_ref())
                        && !is_module(entry.path()))
            })
            .filter_map(Result::ok)
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

                        matches!(root, Some(Component::Normal(_)))
                    })
                    .flat_map(|dependency| {
                        let name = extract_package_name(&dependency.specifier).unwrap();

                        match syntax {
                            Syntax::Typescript(_) => {
                                if dependency.kind == DependencyKind::ImportType {
                                    let type_dependency = "@types/".to_string() + &name;
                                    return if package.dependencies.contains_key(&type_dependency)
                                        || package.dev_dependencies.contains_key(&type_dependency)
                                    {
                                        vec![type_dependency]
                                    } else {
                                        vec![]
                                    };
                                }
                                let type_dependency = extract_type_name(&name);
                                if package.dependencies.contains_key(&type_dependency)
                                    || package.dev_dependencies.contains_key(&type_dependency)
                                {
                                    return vec![name, type_dependency];
                                }
                                vec![name]
                            }
                            _ => vec![name],
                        }
                    })
                    .flat_map(|dependency| {
                        let dependency_module =
                            load_module(&directory.join("node_modules").join(&dependency));
                        let dependencies = match dependency_module {
                            Ok(dependency_module) => iter::once(dependency)
                                .chain(
                                    dependency_module
                                        .peer_dependencies
                                        .keys()
                                        .filter(|&peer_dependency| {
                                            package.dependencies.contains_key(peer_dependency)
                                                || package
                                                    .dev_dependencies
                                                    .contains_key(peer_dependency)
                                        })
                                        .cloned(),
                                )
                                .chain(
                                    dependency_module
                                        .optional_dependencies
                                        .keys()
                                        .filter(|&optional_dependency| {
                                            package.dependencies.contains_key(optional_dependency)
                                                || package
                                                    .dev_dependencies
                                                    .contains_key(optional_dependency)
                                        })
                                        .cloned(),
                                )
                                .collect(),
                            Err(_) => {
                                vec![dependency]
                            }
                        };

                        dependencies
                    })
                    .filter(|dependency| !is_core_module(dependency))
                    .filter(|dependency| {
                        !self.options.ignore_bin_package
                            || !is_bin_dependency(directory, dependency)
                    })
                    .collect();

                let relative_file_path =
                    RelativePathBuf::from_path(file.path().strip_prefix(directory).unwrap())
                        .unwrap();

                (relative_file_path, file_dependencies)
            })
            .collect()
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CheckResult {
    pub package: Package,
    pub directory: PathBuf,
    pub using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
    pub options: CheckerOptions,
}

impl CheckResult {
    pub fn get_missing_dependencies(&self) -> BTreeMap<&str, &HashSet<RelativePathBuf>> {
        if self.options.skip_missing {
            Default::default()
        } else {
            self.using_dependencies
                .iter()
                .filter(|(dependency, _)| {
                    !self
                        .options
                        .get_ignore_matches()
                        .is_match(dependency.as_str())
                })
                .filter(|(dependency, _)| {
                    !self.package.dependencies.contains_key(dependency.as_str())
                        && !self
                            .package
                            .dev_dependencies
                            .contains_key(dependency.as_str())
                        && !self
                            .package
                            .peer_dependencies
                            .contains_key(dependency.as_str())
                        && !self
                            .package
                            .optional_dependencies
                            .contains_key(dependency.as_str())
                })
                .filter(|(dependency, _)| {
                    !self.options.ignore_bin_package
                        || !is_bin_dependency(&self.directory, dependency)
                })
                .map(|(dependency, files)| (dependency.as_str(), files))
                .collect()
        }
    }

    pub fn get_unused_dependencies(&self) -> HashSet<&str> {
        self.package
            .dependencies
            .keys()
            .into_iter()
            .filter(|dependency| {
                !self
                    .options
                    .get_ignore_matches()
                    .is_match(dependency.as_str())
            })
            .filter(|dependency| !self.using_dependencies.contains_key(dependency.as_str()))
            .filter(|dependency| {
                !self.options.ignore_bin_package || !is_bin_dependency(&self.directory, dependency)
            })
            .map(|v| v.as_str())
            .collect()
    }

    pub fn get_unused_dev_dependencies(&self) -> HashSet<&str> {
        self.package
            .dev_dependencies
            .keys()
            .into_iter()
            .filter(|dependency| {
                !self
                    .options
                    .get_ignore_matches()
                    .is_match(dependency.as_str())
            })
            .filter(|dependency| !self.using_dependencies.contains_key(dependency.as_str()))
            .filter(|dependency| {
                !self.options.ignore_bin_package || !is_bin_dependency(&self.directory, dependency)
            })
            .map(|v| v.as_str())
            .collect()
    }
}
