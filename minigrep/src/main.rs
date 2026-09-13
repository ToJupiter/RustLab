use std::{env, process};

use minigrep::file_read::read_poem;
use minigrep::file_read::Config;
use minigrep::file_read::run_config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_config(config) {
        println!("Run config finished with error: {}", e);
        std::process::exit(1);
    };
}

