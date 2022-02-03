use clap::{crate_version, App, AppSettings, Arg};
use std::env;
use std::ffi::OsStr;
use std::io;
use std::path::Path;

use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_dep_graph::analyze_dependencies;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use walkdir::{DirEntry, Error, WalkDir};

pub fn run_cli() {
    let matches = App::new("depcheck")
        .about("The dependency check CLI")
        .bin_name("depcheck")
        .version(crate_version!())
        .arg(
            Arg::new("directory")
                .short('d')
                .takes_value(true)
                .default_value("src")
                .value_name("DIRECTORY")
                .help("Provide a directory"),
        );
    let path = env::current_dir();

    match matches.get_matches().value_of("directory") {
        None => {}
        Some(directory) => {
            for entry in WalkDir::new(directory)
                .into_iter()
                .filter_entry(|entry| {
                    let file_name = entry.file_name().to_string_lossy();
                    file_name != "node_modules"
                        && file_name != "dist"
                        && !file_name.contains("tests")
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
                    analyze_file(entry.path());
                }
            }
        }
    }
}

pub fn analyze_file(path: &Path) {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.load_file(path).expect("failed to load");

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
        .map_err(|mut e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");

    let dependencies = analyze_dependencies(&module, &comments);

    println!("{:#?}", dependencies);

    for dependency in dependencies {
        println!("{:#?}", dependency.specifier);
    }
}
