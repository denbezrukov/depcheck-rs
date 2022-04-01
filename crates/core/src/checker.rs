use std::collections::{BTreeMap, HashSet};

use ignore::overrides::OverrideBuilder;
use ignore::{self, WalkBuilder};
use relative_path::RelativePathBuf;
use swc_common::comments::SingleThreadedComments;
use swc_ecma_dep_graph::analyze_dependencies;

use crate::checker_result::CheckerResult;
use crate::config::Config;
use crate::dependency::Dependency;
use crate::package::{self, Package};
use crate::parser::Parser;
use crate::util::is_module::is_module;
use crate::util::load_module::load_module;

pub struct Checker {
    config: Config,
    parsers: Parser,
}

impl Checker {
    pub fn new(config: Config) -> Self {
        Checker {
            config,
            parsers: Default::default(),
        }
    }
}

impl Checker {
    pub fn check_package(self) -> package::Result<CheckerResult> {
        let package = load_module(self.config.get_directory())?;

        let dependencies = self.check_directory(&package);

        let mut using_dependencies = BTreeMap::new();

        for (file, file_dependencies) in dependencies {
            for dependency in file_dependencies {
                let files = using_dependencies
                    .entry(dependency)
                    .or_insert_with(|| HashSet::with_capacity(100));
                files.insert(file.to_string());
            }
        }

        let result = CheckerResult::new(using_dependencies, package, &self.config);

        Ok(result)
    }

    pub fn check_directory(&self, package: &Package) -> BTreeMap<RelativePathBuf, HashSet<String>> {
        let directory = self.config.get_directory();
        let comments = SingleThreadedComments::default();
        let mut override_builder = OverrideBuilder::new(directory);

        for pattern in self.config.get_ignore_patterns() {
            override_builder
                .add(&format!("!{pattern}"))
                .map_err(|e| format!("Malformed exclude pattern: {e}"))
                .unwrap();
        }

        let overrides = override_builder
            .build()
            .expect("Mismatch in exclude patterns");
        let mut walker = WalkBuilder::new(directory);

        walker.overrides(overrides).filter_entry(move |entry| {
            let is_root_directory = entry.depth() == 0;
            is_root_directory || !is_module(entry.path())
        });

        if let Some(path) = self.config.ignore_path() {
            walker.add_custom_ignore_filename(path);
        }

        let walker = walker.build();

        walker
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| match entry.file_type() {
                Some(file_type) => file_type.is_file(),
                _ => false,
            })
            .filter_map(|file| {
                let path = file.path().strip_prefix(directory).ok()?;
                let relative_file_path = RelativePathBuf::from_path(path).ok()?;
                self.parsers
                    .parse_file(file.path())
                    .map(|(module, syntax)| (relative_file_path, module, syntax))
            })
            .map(|(relative_file_path, module, syntax)| {
                let file_dependencies = analyze_dependencies(&module, &comments);
                let file_dependencies = file_dependencies
                    .into_iter()
                    .map(Dependency::new)
                    .filter(|dependency| dependency.is_external())
                    .flat_map(|dependency| {
                        dependency.extract_dependencies(&syntax, package, &self.config)
                    })
                    .collect();

                (relative_file_path, file_dependencies)
            })
            .collect()
    }
}
