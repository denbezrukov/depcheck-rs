use eyre::WrapErr;
use std::collections::{BTreeMap, HashSet};

use ignore::overrides::OverrideBuilder;
use ignore::{self, WalkBuilder};
use relative_path::RelativePathBuf;
use swc_common::comments::SingleThreadedComments;
use swc_ecma_dep_graph::analyze_dependencies;

use crate::checker_result::CheckerResult;
use crate::config::Config;
use crate::dependency::Dependency;
use crate::package::Package;
use crate::parser::Parser;
use crate::util::is_module::is_module;
use crate::util::load_module::load_module;

pub struct Checker {
    config: Config,
    parsers: Parser,
}

impl Checker {
    pub fn new(config: Config) -> Self {
        log::trace!("init checker with config {:#?}", config);

        Checker {
            config,
            parsers: Default::default(),
        }
    }
}

impl Checker {
    pub fn check_package(self) -> eyre::Result<CheckerResult> {
        let directory = self.config.get_directory();

        log::debug!("checking directory {:#?}", directory);

        let package = load_module(directory)
            .wrap_err_with(|| format!("Failed to read package json from {:?}", directory))?;

        log::debug!("loaded package json {:#?}", package);

        let dependencies = self.check_directory(&package)?;

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

    pub fn check_directory(
        &self,
        package: &Package,
    ) -> eyre::Result<BTreeMap<RelativePathBuf, HashSet<String>>> {
        let directory = self.config.get_directory();
        let comments = SingleThreadedComments::default();
        let mut override_builder = OverrideBuilder::new(directory);

        for pattern in self.config.get_ignore_patterns() {
            override_builder
                .add(&format!("!{pattern}"))
                .wrap_err_with(|| format!("Malformed ignore pattern: {pattern}"))?;
        }

        let overrides = override_builder
            .build()
            .wrap_err_with(|| "Failed to build override builder")?;
        let mut walker = WalkBuilder::new(directory);

        walker.overrides(overrides).filter_entry(move |entry| {
            let is_root_directory = entry.depth() == 0;
            is_root_directory || !is_module(entry.path())
        });

        if let Some(path) = self.config.ignore_path() {
            walker.add_custom_ignore_filename(path);
        }

        let walker = walker.build();

        let result = walker
            .into_iter()
            .filter_map(|entry| {
                log::debug!("walk entry {:#?}", entry);

                entry
                    .map_err(|error| {
                        log::error!("walk error {:#?}", error);
                    })
                    .ok()
            })
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

                log::debug!("analyzed dependencies {:#?}", file_dependencies);

                let file_dependencies = file_dependencies
                    .into_iter()
                    .map(Dependency::new)
                    .filter(|dependency| dependency.is_external())
                    .flat_map(|dependency| {
                        dependency.extract_dependencies(&syntax, package, &self.config)
                    })
                    .collect();

                log::debug!("extracted dependencies {:#?}", file_dependencies);

                (relative_file_path, file_dependencies)
            })
            .collect();

        Ok(result)
    }
}
