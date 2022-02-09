use crate::package::{self, Package};
use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_dep_graph::{analyze_dependencies, DependencyDescriptor};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use walkdir::WalkDir;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CheckResult {
    pub using_dependencies: HashSet<String>,
    pub unused_dependencies: HashSet<String>,
    pub missing_dependencies: HashSet<String>,
}

pub fn check_package(directory: PathBuf) -> package::Result<CheckResult> {
    let mut package_path = directory.to_owned();
    package_path.push("package.json");

    let package = Package::from_path(package_path)?;
    let using_dependencies = check_directory(directory);

    let package_dependencies: HashSet<String> = package
        .dependencies
        .keys()
        .map(|key| key.to_string())
        .collect();

    let unused_dependencies = package_dependencies
        .difference(&using_dependencies)
        .cloned()
        .collect();

    let missing_dependencies = using_dependencies
        .difference(&package_dependencies)
        .cloned()
        .collect();

    Ok(CheckResult {
        using_dependencies,
        unused_dependencies,
        missing_dependencies,
    })
}

pub fn check_directory(directory: PathBuf) -> HashSet<String> {
    let mut dependencies = Vec::with_capacity(1000);

    for entry in WalkDir::new(directory)
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
                extension == "ts" || extension == "tsx"
            }
        })
    {
        let file_dependencies = check_file(entry.path());
        dependencies.push(file_dependencies);
    }

    dependencies
        .iter()
        .flatten()
        .flat_map(|dependency| {
            let dependency = PathBuf::from(dependency.specifier.to_string());
            let root = dependency.components().next()?;
            match root {
                Component::Normal(root) => Some(root.to_str()?.to_string()),
                _ => None,
            }
        })
        .collect()
}

pub fn check_file(file: &Path) -> Vec<DependencyDescriptor> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.load_file(file).expect("failed to load");

    let comments = SingleThreadedComments::default();
    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");

    analyze_dependencies(&module, &comments)
}
