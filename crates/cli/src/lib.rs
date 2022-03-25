use clap::{crate_authors, crate_version, Arg, Command};

pub fn build_app() -> Command<'static> {
    let app = Command::new("depcheck-rs")
        .author(crate_authors!())
        .version(crate_version!())
        .about("The dependency check CLI")
        .bin_name("depcheck-rs")
        .arg(
            Arg::new("directory")
                .long("directory")
                .short('d')
                .takes_value(true)
                .default_value(".")
                .value_name("DIRECTORY")
                .help("The directory argument is the root directory of your project"),
        )
        .arg(
            Arg::new("ignore-bin-package")
                .long("ignore-bin-package")
                .takes_value(true)
                .default_value("false")
                .help("A flag to indicate if depcheck ignores the packages containing bin entry"),
        )
        .arg(
            Arg::new("skip-missing")
                .long("skip-missing")
                .takes_value(true)
                .default_value("false")
                .help("A flag to indicate if depcheck skips calculation of missing dependencies"),
        )
        .arg(
            Arg::new("ignore-path")
                .long("ignore-path")
                .takes_value(true)
                .help("Path to a file with patterns describing files to ignore"),
        );
    app
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_app() {
        use super::build_app;

        build_app().debug_assert();
    }
}
