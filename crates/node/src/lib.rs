#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::path::PathBuf;

use depckeck_rs_core::checker::Checker;
use depckeck_rs_core::config;

#[napi(object)]
pub struct Options {
    pub ignore_bin_package: Option<bool>,
    pub ignore_patterns: Option<Vec<String>>,
    pub ignore_matches: Option<Vec<String>>,
    pub skip_missing: Option<bool>,
    pub ignore_path: Option<String>,
}

#[napi]
pub fn sum(v: i32) -> String {
    String::from("12321")
}

#[napi]
pub fn depcheck(path: String, options: Options) -> String {
    let path = PathBuf::from(path);

    let mut config = config::Config::new(path);

    if let Some(ignore_bin_package) = options.ignore_bin_package {
        config = config.with_ignore_bin_package(ignore_bin_package);
    }

    if let Some(ignore_patterns) = options.ignore_patterns {
        config = config.with_ignore_patterns(ignore_patterns);
    }

    if let Some(ignore_matches) = options.ignore_matches {
        config = config.with_ignore_matches(ignore_matches);
    }

    if let Some(skip_missing) = options.skip_missing {
        config = config.with_skip_missing(skip_missing);
    }

    let ignore_path = options.ignore_path.map(PathBuf::from);
    config = config.with_ignore_path(ignore_path);

    let result = Checker::new(config).check_package().unwrap();

    result.to_json().unwrap()
}
