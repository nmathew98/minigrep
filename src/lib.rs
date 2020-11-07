use std::env;
use std::error::Error;
use std::fs;

// Stores the configuration details
// query: string we're searching for
// filename: file we're searching in
// case_sensitive: use case sensitive search?
pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		// First argument is the path
		args.next();

		// Get the query and the filename
		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Please specify a query"),
		};
		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Please specify a filename"),
		};
		// Check if case sensitive search is on (the converse in actuality)
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config {
			query,
			filename,
			case_sensitive,
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	// Read the contents of the file into a string
	let contents = fs::read_to_string(config.filename)?;

	// Search based on if we want case sensitive or case insensitive search
	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}
	Ok(())
}

// Case sensitive search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}

// Case insensitive search
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	// Test the case sensitive search
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	// Test the case insensitive search
	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
