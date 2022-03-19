use globset::{self, Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config {
    directory: PathBuf,
    ignore_bin_package: bool,
    ignore_patterns: Vec<String>,
    ignore_matches: Vec<String>,
    skip_missing: bool,
    read_depcheckignore: bool,
}

impl Config {
    pub fn new(directory: PathBuf) -> Self {
        let ignore_patterns = [
            r".git",
            r".svn",
            r".hg",
            r".idea",
            r"node_modules",
            r"dist",
            r"build",
            r"bower_components",
            // Images
            r"*.png",
            r"*.gif",
            r"*.jpg",
            r"*.jpeg",
            r"*.svg",
            // Fonts
            r"*.woff",
            r"*.woff2",
            r"*.eot",
            r"*.ttf",
            // Archives
            r"*.zip",
            r"*.gz",
            // Videos
            r"*.mp4",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        Config {
            directory,
            ignore_patterns,
            skip_missing: false,
            ignore_bin_package: false,
            ignore_matches: Vec::new(),
            read_depcheckignore: false,
        }
    }
}

impl Config {
    pub fn with_ignore_patterns(mut self, ignore_patterns: Vec<String>) -> Self {
        self.ignore_patterns = ignore_patterns;
        self
    }

    pub fn with_skip_missing(mut self, skip_missing: bool) -> Self {
        self.skip_missing = skip_missing;
        self
    }

    pub fn with_ignore_bin_package(mut self, ignore_bin_package: bool) -> Self {
        self.ignore_bin_package = ignore_bin_package;
        self
    }

    pub fn with_ignore_matches(mut self, ignore_matches: Vec<String>) -> Self {
        self.ignore_matches = ignore_matches;
        self
    }

    pub fn with_read_depcheckignore(mut self, read_depcheckignore: bool) -> Self {
        self.read_depcheckignore = read_depcheckignore;
        self
    }

    pub fn ignore_bin_package(&self) -> bool {
        self.ignore_bin_package
    }

    pub fn skip_missing(&self) -> bool {
        self.skip_missing
    }

    pub fn read_depcheckignore(&self) -> bool {
        self.read_depcheckignore
    }
}

impl Config {
    pub fn get_ignore_patterns(&self) -> &Vec<String> {
        &self.ignore_patterns
    }

    pub fn get_ignore_matches(&self) -> Result<GlobSet, globset::Error> {
        let mut builder = GlobSetBuilder::new();

        for pattern in &self.ignore_matches {
            builder.add(Glob::new(pattern.as_str())?);
        }

        builder.build()
    }

    pub fn get_directory(&self) -> &Path {
        &self.directory
    }
}