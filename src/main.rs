use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // Take arguments and store the specified configuration
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // Throw an error if there are no arguments passed
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("{}", e);

        process::exit(1);
    }
}
