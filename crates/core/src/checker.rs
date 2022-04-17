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
use crossbeam::channel;
use std::path::PathBuf;

/// Dependencies checker.
#[derive(Clone, Debug, Eq, PartialEq)]
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

pub enum WorkerResult {
    Entry(PathBuf),
    Error(ignore::Error),
}

impl Checker {
    /// check dependencies with config and parsers.
    pub fn check_package(self) -> eyre::Result<CheckerResult> {
        let directory = self.config.get_directory();

        log::debug!("checking directory {:#?}", directory);

        let package = load_module(directory)
            .wrap_err_with(|| format!("Failed to read package json from {:?}", directory))?;

        log::debug!("loaded package json {:#?}", package);

        let using_dependencies = self.check_directory(&package)?;

        let result = CheckerResult::new(using_dependencies, package, &self.config);

        Ok(result)
    }

    fn check_directory(
        &self,
        package: &Package,
    ) -> eyre::Result<BTreeMap<String, HashSet<String>>> {
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

        walker.overrides(overrides);

        if let Some(path) = self.config.ignore_path() {
            walker.add_custom_ignore_filename(path);
        }

        let (tx, rx) = channel::unbounded::<WorkerResult>();

        let nums_of_thread = num_cpus::get();

        walker.threads(nums_of_thread).build_parallel().run(|| {
            let tx = tx.clone();
            Box::new(move |entry| {
                log::debug!("walk entry {:#?}", entry);
                return match entry {
                    Ok(ref entry) => {
                        if entry.depth() == 0 {
                            return ignore::WalkState::Continue;
                        }

                        if is_module(entry.path()) {
                            return ignore::WalkState::Skip;
                        }

                        if let Some(file_type) = entry.file_type() {
                            if file_type.is_file() {
                                let worker_result = WorkerResult::Entry(entry.path().to_owned());
                                return match tx.send(worker_result) {
                                    Ok(_) => ignore::WalkState::Continue,
                                    Err(_) => ignore::WalkState::Quit,
                                };
                            }
                        }

                        ignore::WalkState::Continue
                    }
                    Err(error) => {
                        log::error!("walk error {:#?}", error);

                        return match tx.send(WorkerResult::Error(error)) {
                            Ok(_) => ignore::WalkState::Continue,
                            Err(_) => ignore::WalkState::Quit,
                        };
                    }
                };
            })
        });

        drop(tx);

        let mut using_dependencies = BTreeMap::new();

        while let Ok(message) = rx.try_recv() {
            match message {
                WorkerResult::Entry(path) => {
                    let file = path
                        .strip_prefix(directory)
                        .map(|path| RelativePathBuf::from_path(path).ok())
                        .ok()
                        .flatten();
                    let file_dependencies =
                        self.parsers.parse_file(&path).map(|(module, syntax)| {
                            analyze_dependencies(&module, &comments)
                                .into_iter()
                                .map(Dependency::new)
                                .filter(|dependency| dependency.is_external())
                                .flat_map(|dependency| {
                                    dependency.extract_dependencies(&syntax, package, &self.config)
                                })
                                .collect::<HashSet<_>>()
                        });

                    if let (Some(file), Some(file_dependencies)) = (file, file_dependencies) {
                        for dependency in file_dependencies {
                            let files = using_dependencies
                                .entry(dependency)
                                .or_insert_with(|| HashSet::with_capacity(100));
                            files.insert(file.to_string());
                        }
                    }
                }
                WorkerResult::Error(error) => return Err(eyre::Report::from(error)),
            }
        }

        Ok(using_dependencies)
    }
}
