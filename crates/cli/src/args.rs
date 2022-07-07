use std::path::{Path, PathBuf};
use clap::builder::ValueParser;

/// The dependencies checker CLI arguments.
#[derive(Debug, clap::Parser)]
#[clap(bin_name = "depcheck-rs")]
#[clap(about = "The dependency check CLI")]
#[clap(author = clap::crate_authors!())]
#[clap(version = clap::crate_version!())]
pub struct Args {
    /// The directory argument is the root directory of your project.
    #[clap(long, short = 'd')]
    #[clap(help = "The directory argument is the root directory of your project")]
    #[clap(default_value = ".")]
    #[clap(takes_value = true)]
    #[clap(value_parser = ValueParser::os_string())]
    #[clap(value_parser = validate_directory)]
    pub directory: PathBuf,

    /// A flag to indicate if depcheck ignores the packages containing bin entry.
    #[clap(long = "ignore-bin-package")]
    #[clap(help = "A flag to indicate if depcheck ignores the packages containing bin entry")]
    #[clap(value_parser)]
    pub ignore_bin_package: bool,

    /// A flag to indicate if depcheck skips calculation of missing dependencies.
    #[clap(long = "skip-missing")]
    #[clap(help = "A flag to indicate if depcheck skips calculation of missing dependencies")]
    #[clap(value_parser)]
    pub skip_missing: bool,

    /// Path to a file with patterns describing files to ignore.
    #[clap(long = "ignore-path")]
    #[clap(help = "Path to a file with patterns describing files to ignore")]
    #[clap(takes_value = true)]
    #[clap(value_parser = ValueParser::os_string())]
    pub ignore_path: Option<PathBuf>,

    /// Comma separated patterns describing files or directories to ignore.
    #[clap(long = "ignore-patterns")]
    #[clap(help = "Comma separated patterns describing files or directories to ignore")]
    #[clap(use_value_delimiter = true)]
    #[clap(value_parser)]
    pub ignore_patterns: Option<Vec<String>>,

    /// A comma separated array containing package names to ignore.
    #[clap(long = "ignore_matches")]
    #[clap(help = "A comma separated array containing package names to ignore")]
    #[clap(use_value_delimiter = true)]
    #[clap(value_parser)]
    pub ignore_matches: Option<Vec<String>>,

    /// logging level
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

fn is_existing_directory(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.canonicalize().is_ok())
}

/// validation function for directory argument
fn validate_directory(path: &str) -> eyre::Result<PathBuf> {
    let path = PathBuf::from(path);
    if is_existing_directory(&path) {
        Ok(path)
    } else {
        Err(eyre::eyre!("directory doesn't exist."))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
