use std::env;
use std::fs;

fn main() {
    // Take arguments from the command line
    let args: Vec<String> = env::args().collect();

    // Store a reference to the query and filename from the arguments passed
    let query = &args[1];
    let filename = &args[2];

    // Read the file
    let contents = fs::read_to_string(filename)
        .expect(&format!("Something went wrong while reading `{}`", filename)[..]);

    println!("Searching for `{}` in file `{}`", query, filename);
}
