use clap::{crate_version, App, Arg};

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

    println!("{:?}", matches);
}
