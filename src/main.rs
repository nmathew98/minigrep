use std::env;

fn main() {
    // Take arguments from the command line
    let args: Vec<String> = env::args().collect();
    // Store a reference to the query and filename from the arguments passed
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for `{}` in file `{}`", query, filename);
}
