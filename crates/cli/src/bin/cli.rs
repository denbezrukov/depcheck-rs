use clap::Parser;
use depcheck_rs_cli::Args;
use depckeck_rs_core::checker::Checker;
use depckeck_rs_core::config::Config;
use proc_exit::WithCodeResultExt;

fn main() {
    human_panic::setup_panic!();
    let result = run();
    proc_exit::exit(result);
}

fn run() -> proc_exit::ExitResult {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) if e.use_stderr() => {
            let _ = e.print();
            return proc_exit::Code::USAGE_ERR.ok();
        }
        Err(e) => {
            let _ = e.print();
            return proc_exit::Code::SUCCESS.ok();
        }
    };

    let Args {
        directory,
        ignore_bin_package,
        skip_missing,
        ignore_path,
        ignore_patterns,
        ignore_matches,
        verbose,
    } = args;

    env_logger::Builder::new()
        .filter_level(verbose.log_level_filter())
        .init();

    let mut config = Config::new(directory)
        .with_ignore_bin_package(ignore_bin_package)
        .with_skip_missing(skip_missing)
        .with_ignore_path(ignore_path);

    if let Some(ignore_patterns) = ignore_patterns {
        config = config.with_ignore_patterns(ignore_patterns);
    }

    if let Some(ignore_matches) = ignore_matches {
        config = config.with_ignore_matches(ignore_matches);
    }

    let result = Checker::new(config)
        .check_package()
        .with_code(proc_exit::Code::USAGE_ERR)?;

    println!("{:#?}", result);

    Ok(())
}
