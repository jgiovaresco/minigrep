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
}

fn main() {
    let args = Cli::from_args();
    let config = Config {
        pattern: args.pattern,
        path: args.path,
    };

    println!(
        "Searching for '{}', in file '{}':\n",
        config.pattern,
        config.path.display()
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
