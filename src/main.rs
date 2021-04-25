use clap_verbosity_flag::Verbosity;
use env_logger::Builder as LoggerBuilder;
use log::{debug, LevelFilter};
use std::process;
use structopt::StructOpt;

use minigrep::Config;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() {
    let args = Cli::from_args();
    let level_filter = match args.verbose.log_level() {
        Some(level) => level.to_level_filter(),
        None => LevelFilter::Off,
    };
    LoggerBuilder::new()
        .filter(None, level_filter)
        .try_init()
        .unwrap();

    let config = Config {
        pattern: args.pattern,
        path: args.path,
    };

    debug!(
        "Searching for '{}', in file '{}':\n",
        config.pattern,
        config.path.display()
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
