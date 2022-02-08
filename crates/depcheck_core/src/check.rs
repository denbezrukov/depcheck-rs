use std::path::Path;
use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};
use swc_ecma_dep_graph::{analyze_dependencies, DependencyDescriptor, DependencyKind};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use walkdir::WalkDir;
use crate::package::{self, Package};

pub fn check_package(directory: &Path) -> package::Result<()> {
    let mut package_path = directory.to_owned();
    package_path.push("package.json");
    let package = Package::from_path(package_path)?;

    check_directory(directory);
    Ok(())
}

pub fn check_directory(directory: &Path) {
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
        if entry.file_type().is_file() {
            let file_dependencies = check_file(entry.path());
            dependencies.push(file_dependencies);
        }
    }

    dependencies.iter().flatten().for_each(|dependency| {
        println!("{:?}", dependency);
    });
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

    let dependencies = analyze_dependencies(&module, &comments);

    dependencies
}
