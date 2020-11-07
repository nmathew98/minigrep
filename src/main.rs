use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // Take arguments from the command line
    let args: Vec<String> = env::args().collect();

    // Store a reference to the query and filename from the arguments passed
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("{}", e);

        process::exit(1);
    }
}
