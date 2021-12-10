extern crate file_read;

use std::env;
use std::process;

use file_read::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("arguments parsing error: {}", err);
        process::exit(1);
    });

    if let Err(e) = file_read::run(config) {
        eprintln!("application error: {}", e);
        process::exit(0);
    }
}

