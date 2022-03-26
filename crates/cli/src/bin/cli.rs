use depcheck_rs_cli::build_app;
use depckeck_rs_core::checker::Checker;
use depckeck_rs_core::config::Config;
use depckeck_rs_core::package::{self};

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("directory not found")]
    DirectoryNotFound,

    #[error("ignore file not found")]
    IgnoreFileNotFound,

    #[error("failed to parse arguments")]
    Clap(#[from] clap::Error),

    #[error("failed to read package")]
    Package(#[from] package::Error),

    #[error("failed to serialize result")]
    SerializeResult(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let app = build_app();

    let matches = app.get_matches();

    let directory = extract_directory(&matches)?;
    let ignore_bin_package = extract_ignore_bin_package(&matches)?;
    let skip_missing = extract_skip_missing(&matches)?;
    let ignore_path = extract_ignore_path(&matches)?;

    let config = Config::new(directory)
        .with_ignore_bin_package(ignore_bin_package)
        .with_skip_missing(skip_missing)
        .with_ignore_path(ignore_path);

    let result = Checker::new(config).check_package()?;

    let json = result.to_json()?;

    println!("{:#?}", json);

    Ok(())
}

fn extract_directory(matches: &clap::ArgMatches) -> Result<PathBuf> {
    let directory = matches
        .value_of_os("directory")
        .unwrap_or_else(|| OsStr::new("."));
    let path = PathBuf::from(directory);
    if is_existing_directory(&path) {
        Ok(path)
    } else {
        Err(Error::DirectoryNotFound)
    }
}

fn is_existing_directory(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.canonicalize().is_ok())
}

fn extract_ignore_bin_package(matches: &clap::ArgMatches) -> Result<bool> {
    Ok(matches.value_of_t::<bool>("ignore-bin-package")?)
}

fn extract_skip_missing(matches: &clap::ArgMatches) -> Result<bool> {
    Ok(matches.value_of_t::<bool>("skip-missing")?)
}

fn extract_ignore_path(matches: &clap::ArgMatches) -> Result<Option<PathBuf>> {
    matches
        .value_of_os("ignore-path")
        .map(PathBuf::from)
        .map(|path| {
            if is_existing_file(&path) {
                Ok(path)
            } else {
                Err(Error::IgnoreFileNotFound)
            }
        })
        .transpose()
}

fn is_existing_file(path: &Path) -> bool {
    path.is_file() && (path.file_name().is_some() || path.canonicalize().is_ok())
}

// impl From<CheckResult> for CliResult {
//     fn from(check_result: CheckResult) -> Self {
//         // let a: BTreeMap<String, HashSet<String>> = check_result.using_dependencies.into();
//         todo!();
//         // CliResult {
//         //     using_dependencies: check_result.using_dependencies.into(),
//         // }
//     }
// }

// #[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
// struct CliResult<'a> {
//     using_dependencies: BTreeMap<String, HashSet<RelativePathBuf>>,
//     missing_dependencies: BTreeMap<&'a str, &'a HashSet<RelativePathBuf>>,
//     unused_dependencies: HashSet<&'a str>,
//     unused_dev_dependencies: HashSet<&'a str>,
// }
