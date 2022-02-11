use std::collections::{BTreeMap, HashSet};
use std::path::{Component, Path, PathBuf};

use relative_path::RelativePathBuf;
use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_dep_graph::{analyze_dependencies, DependencyDescriptor};
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};
use walkdir::WalkDir;

use crate::package::{self, Package};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CheckResult {
    pub using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
    pub unused_dependencies: HashSet<String>,
    pub missing_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
}

pub fn check_package(directory: PathBuf) -> package::Result<CheckResult> {
    let mut package_path = directory.to_owned();
    package_path.push("package.json");

    let package = Package::from_path(package_path)?;

    let dependencies = check_directory(directory);

    let mut using_dependencies = BTreeMap::new();

    dependencies.into_iter().for_each(|(path, dependencies)| {
        dependencies.iter().for_each(|dependency| {
            let files = using_dependencies
                .entry(dependency.clone())
                .or_insert(HashSet::with_capacity(100));
            files.insert(path.clone());
        })
    });

    let missing_dependencies = using_dependencies
        .iter()
        .filter(|(dependency, _)| package.dependencies.contains_key(*dependency))
        .map(|(dependency, files)| (dependency.clone(), files.clone()))
        .collect();

    let package_dependencies: HashSet<&String> = package.dependencies.keys().collect();
    let exclusive_using_dependencies: HashSet<&String> = using_dependencies.keys().collect();

    let unused_dependencies = package_dependencies
        .difference(&exclusive_using_dependencies)
        .cloned()
        .cloned()
        .collect();

    Ok(CheckResult {
        using_dependencies,
        unused_dependencies,
        missing_dependencies,
    })
}

pub fn check_directory(directory: PathBuf) -> BTreeMap<RelativePathBuf, HashSet<String>> {
    let mut dependencies = BTreeMap::new();

    WalkDir::new(&directory)
        .into_iter()
        .filter_entry(|entry| {
            let file_name = entry.file_name().to_string_lossy();
            file_name != "node_modules" && file_name != "dist"
        })
        .filter_map(|entry| Result::ok(entry))
        .filter(|dir_entry| dir_entry.file_type().is_file())
        .filter(|file| match file.path().extension() {
            None => {
                return false;
            }
            Some(extension) => {
                let extension = extension.to_string_lossy();
                // extension == "ts" || extension == "tsx"
                extension == "js" || extension == "jsx"
            }
        })
        .for_each(|entry| {
            let file_dependencies = check_file(entry.path());
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
                RelativePathBuf::from_path(entry.path().strip_prefix(&directory).unwrap()).unwrap();
            dependencies.insert(relative_file_path.to_owned(), file_dependencies);
        });
    dependencies
}

pub fn check_file(file: &Path) -> Vec<DependencyDescriptor> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.load_file(file).expect("failed to load");

    let comments = SingleThreadedComments::default();
    // let lexer = Lexer::new(
    //     Syntax::Typescript(TsConfig {
    //         tsx: true,
    //         ..Default::default()
    //     }),
    //     Default::default(),
    //     StringInput::from(&*fm),
    //     Some(&comments),
    // );
    let lexer = Lexer::new(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    for error in parser.take_errors() {
        error.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");

    analyze_dependencies(&module, &comments)
}
