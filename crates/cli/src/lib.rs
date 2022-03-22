use clap::{crate_version, Arg, Command};

pub fn run_cli() {
    let app = Command::new("depcheck-rs")
        .about("The dependency check CLI")
        .bin_name("depcheck-rs")
        .version(crate_version!())
        .arg(
            Arg::new("directory")
                .short('d')
                .takes_value(true)
                .value_name("DIRECTORY")
                .help("Provide a directory"),
        );

    println!("{:#?}", app.get_matches());
}
