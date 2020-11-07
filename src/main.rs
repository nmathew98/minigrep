use std::env;
use std::fs;
use std::process;

fn main() {
    // Take arguments from the command line
    let args: Vec<String> = env::args().collect();

    // Store a reference to the query and filename from the arguments passed
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!(
        "Searching for `{}` in file `{}`",
        config.query, config.filename
    );

    // Read the file
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong while reading the file");
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please provide a query and a filename");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
