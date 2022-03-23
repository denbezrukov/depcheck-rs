use std::env;
use clap::{crate_version, crate_authors, Arg, Command};

pub fn run_cli() {
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
                .value_name("DIRECTORY")
                .help("Provide a directory"),
        )
        .arg(
            Arg::new("ignore-bin-package")
                .long("ignore-bin-package")
                .default_value("false"),
        );

    let matches = app.get_matches();
    let a: bool = matches.value_of_t("ignore-bin-package").unwrap();
    println!("{:#?}", a);
}
