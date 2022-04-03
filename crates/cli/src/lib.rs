use clap::Command;
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Parser)]
#[clap(bin_name = "depcheck-rs")]
#[clap(about = "The dependency check CLI")]
#[clap(author = clap::crate_authors!())]
#[clap(version = clap::crate_version!())]
pub struct Args {
    #[clap(long, short = 'd')]
    #[clap(help = "The directory argument is the root directory of your project")]
    #[clap(default_value = ".")]
    #[clap(parse(from_os_str))]
    #[clap(takes_value = true)]
    #[clap(allow_invalid_utf8 = true)]
    #[clap(validator = validate_directory)]
    pub directory: PathBuf,

    #[clap(long = "ignore-bin-package")]
    #[clap(help = "A flag to indicate if depcheck ignores the packages containing bin entry")]
    pub ignore_bin_package: bool,

    #[clap(long = "skip-missing")]
    #[clap(help = "A flag to indicate if depcheck skips calculation of missing dependencies")]
    pub skip_missing: bool,

    #[clap(long = "ignore-path")]
    #[clap(help = "Path to a file with patterns describing files to ignore")]
    #[clap(parse(from_os_str))]
    #[clap(takes_value = true)]
    #[clap(allow_invalid_utf8 = true)]
    pub ignore_path: Option<PathBuf>,

    #[clap(long = "ignore-patterns")]
    #[clap(help = "Comma separated patterns describing files or directories to ignore")]
    #[clap(use_value_delimiter = true)]
    pub ignore_patterns: Option<Vec<String>>,

    #[clap(long = "ignore_matches")]
    #[clap(help = "A comma separated array containing package names to ignore")]
    #[clap(use_value_delimiter = true)]
    pub ignore_matches: Option<Vec<String>>,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

fn is_existing_directory(path: &Path) -> bool {
    path.is_dir() && (path.file_name().is_some() || path.canonicalize().is_ok())
}

fn validate_directory(path: &str) -> eyre::Result<()> {
    let path = PathBuf::from(path);
    if is_existing_directory(&path) {
        Ok(())
    } else {
        Err(eyre::eyre!("directory doesn't exist."))
    }
}

pub fn get_command() -> Command<'static> {
    use clap::CommandFactory;
    Args::command()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        get_command().debug_assert()
    }
}
