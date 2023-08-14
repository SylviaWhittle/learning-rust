use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Note that env::args() returns an iterator.
    // Rather than collecting the iterator values into a vector and passing them
    // in, we can pass ownership of the iterator returned from env::args() to
    // Config::build directly.
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    }
}
