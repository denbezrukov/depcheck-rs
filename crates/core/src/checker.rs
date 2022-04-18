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
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

/// Dependencies checker.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checker {
    config: Arc<Config>,
    parser: Arc<Parser>,
}

impl Checker {
    pub fn new(config: Config) -> Self {
        log::trace!("init checker with config {:#?}", config);

        Checker {
            config: Arc::new(config),
            parser: Default::default(),
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

        let using_dependencies = self.check_directory(Arc::new(package.clone()))?;

        let result = CheckerResult::new(using_dependencies, package, &self.config);

        Ok(result)
    }

    fn check_directory(
        &self,
        package: Arc<Package>,
    ) -> eyre::Result<BTreeMap<String, HashSet<String>>> {
        let directory = self.config.get_directory();
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

        let (file_sender, file_receiver) = mpsc::channel();
        let (dependency_sender, dependency_receiver) = mpsc::channel();

        let nums_of_thread = num_cpus::get();
        let parallel_walker = walker.threads(nums_of_thread).build_parallel();

        let mut using_dependencies = BTreeMap::new();

        let handle = spawn_dependency_senders(
            file_receiver,
            dependency_sender,
            &self.config,
            &self.parser,
            &package,
        );

        spawn_file_senders(parallel_walker, file_sender);

        handle.join().unwrap();

        while let Ok((file, file_dependencies)) = dependency_receiver.recv() {
            for dependency in file_dependencies {
                let files = using_dependencies
                    .entry(dependency)
                    .or_insert_with(|| HashSet::with_capacity(100));
                files.insert(file.to_string());
            }
        }

        Ok(using_dependencies)
    }
}

fn spawn_file_senders(parallel_walker: ignore::WalkParallel, file_sender: Sender<WorkerResult>) {
    parallel_walker.run(|| {
        let file_sender = file_sender.clone();
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
                            return match file_sender.send(worker_result) {
                                Ok(_) => ignore::WalkState::Continue,
                                Err(_) => ignore::WalkState::Quit,
                            };
                        }
                    }

                    ignore::WalkState::Continue
                }
                Err(error) => {
                    log::error!("walk error {:#?}", error);

                    return match file_sender.send(WorkerResult::Error(error)) {
                        Ok(_) => ignore::WalkState::Continue,
                        Err(_) => ignore::WalkState::Quit,
                    };
                }
            };
        })
    });
}

fn spawn_dependency_senders(
    file_receiver: Receiver<WorkerResult>,
    dependency_sender: Sender<(RelativePathBuf, HashSet<String>)>,
    config: &Arc<Config>,
    parser: &Arc<Parser>,
    package: &Arc<Package>,
) -> thread::JoinHandle<Vec<()>> {
    let config = Arc::clone(config);
    let parser = Arc::clone(parser);
    let package = Arc::clone(package);
    thread::spawn(move || {
        let shared_file_receiver = Arc::new(Mutex::new(file_receiver));

        let nums_of_thread = num_cpus::get();
        let mut handles = Vec::with_capacity(nums_of_thread);

        for _ in 0..nums_of_thread {
            let file_receiver = Arc::clone(&shared_file_receiver);
            let config = Arc::clone(&config);
            let parser = Arc::clone(&parser);
            let package = Arc::clone(&package);
            let dependency_sender = dependency_sender.clone();

            let handle = thread::spawn(move || loop {
                let lock = file_receiver.lock().unwrap();

                let path: PathBuf = match lock.recv() {
                    Ok(WorkerResult::Entry(path)) => path,
                    Ok(WorkerResult::Error(_)) => {
                        continue;
                    }
                    Err(_) => break,
                };

                drop(lock);
                let comments = SingleThreadedComments::default();

                let file = path
                    .strip_prefix(config.get_directory())
                    .map(|path| RelativePathBuf::from_path(path).ok())
                    .ok()
                    .flatten();
                let file_dependencies = parser.parse_file(&path).map(|(module, syntax)| {
                    analyze_dependencies(&module, &comments)
                        .into_iter()
                        .map(Dependency::new)
                        .filter(|dependency| dependency.is_external())
                        .flat_map(|dependency| {
                            dependency.extract_dependencies(&syntax, &package, &config)
                        })
                        .collect::<HashSet<_>>()
                });

                if let (Some(file), Some(file_dependencies)) = (file, file_dependencies) {
                    dependency_sender.send((file, file_dependencies)).unwrap();
                }
            });

            handles.push(handle);
        }

        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>()
    })
}
