use crate::config::Config;
use crate::package::Package;
use crate::util::extract_type_name::extract_type_name;
use crate::util::is_bin_dependency::is_bin_dependency;
use crate::util::is_core_module::is_core_module;
use crate::util::load_module::load_module;
use regex::Regex;
use std::iter;
use std::path::{Component, PathBuf};
use swc_ecma_dep_graph::{DependencyDescriptor, DependencyKind};
use swc_ecma_parser::Syntax;

pub struct Dependency {
    descriptor: DependencyDescriptor,
}

impl Dependency {
    pub fn new(descriptor: DependencyDescriptor) -> Self {
        Dependency { descriptor }
    }
}

impl Dependency {
    pub fn extract_dependencies(
        self,
        syntax: &Syntax,
        package: &Package,
        config: &Config,
    ) -> Vec<String> {
        self.get_dependencies(syntax, package)
            .into_iter()
            .filter(|dependency| !is_core_module(dependency.as_str()))
            .filter(|dependency| {
                !config.ignore_bin_package()
                    || !is_bin_dependency(config.get_directory(), dependency)
            })
            .flat_map(|dependency| {
                let dependency_module = load_module(
                    &config
                        .get_directory()
                        .join("node_modules")
                        .join(&dependency),
                );
                dependency_module
                    .map(|dependency_module| {
                        iter::once(&dependency)
                            .chain(dependency_module.peer_dependencies.keys().filter(
                                |&peer_dependency| {
                                    package.is_dependency(peer_dependency)
                                        || package.is_dev_dependency(peer_dependency)
                                },
                            ))
                            .chain(dependency_module.optional_dependencies.keys().filter(
                                |&optional_dependency| {
                                    package.is_dependency(optional_dependency)
                                        || package.is_dev_dependency(optional_dependency)
                                },
                            ))
                            .cloned()
                            .collect()
                    })
                    .unwrap_or_else(|_| vec![dependency])
            })
            .collect()
    }

    fn get_dependencies(&self, syntax: &Syntax, package: &Package) -> Vec<String> {
        self.get_dependency()
            .map(|dependency| match syntax {
                Syntax::Typescript(_) => {
                    if self.descriptor.kind == DependencyKind::ImportType {
                        let dependency_type = format!("@types/{dependency}");
                        if package.is_dependency(&dependency_type)
                            || package.is_dev_dependency(&dependency_type)
                        {
                            vec![dependency_type]
                        } else {
                            vec![]
                        }
                    } else {
                        let dependency_type = extract_type_name(&dependency);
                        if package.is_dependency(&dependency_type)
                            || package.is_dev_dependency(&dependency_type)
                        {
                            vec![dependency, dependency_type]
                        } else {
                            vec![dependency]
                        }
                    }
                }
                _ => {
                    vec![dependency]
                }
            })
            .unwrap_or_default()
    }

    fn get_dependency(&self) -> Option<String> {
        let specifier = self.descriptor.specifier.to_string();
        let scope_pattern = Regex::new(r"^(?:(@[^/]+)[/]+)([^/]+)[/]?").unwrap();
        let base_pattern = Regex::new(r"^([^/]+)[/]?").unwrap();
        let scope_pattern_test = Regex::new(r"^@").unwrap();

        if scope_pattern_test.is_match(&specifier) {
            let captures = scope_pattern.captures(&specifier)?;

            return match (captures.get(1), captures.get(2)) {
                (Some(first), Some(second)) => {
                    Some(first.as_str().to_string() + "/" + second.as_str())
                }
                _ => None,
            };
        } else {
            let captures = base_pattern.captures(&specifier)?;
            captures.get(1).map(|v| v.as_str().to_string())
        }
    }

    pub fn is_external(&self) -> bool {
        let path = PathBuf::from(self.descriptor.specifier.to_string());
        let root_component = path.components().next();
        matches!(root_component, Some(Component::Normal(_)))
    }
}
