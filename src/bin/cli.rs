use clap::{crate_version, Parser};
use depcheck_cli::run_cli;

#[derive(Parser)]
#[clap(author)]
#[clap(version)]
#[clap(bin_name = "depcheck")]
#[clap(about = "The dependency check CLI", long_about = None)]
struct Cli {
    #[clap(short = 'c')]
    #[clap(long)]
    #[clap(default_value = "src")]
    #[clap(value_name = "DIRECTORY")]
    #[clap(help = "Provide a directory")]
    directory: String,
}

fn main() {
    let cli = Cli::parse();

    println!("directory: {:?}", cli.directory);
}
