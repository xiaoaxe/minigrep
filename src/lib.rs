use std::error::Error;
use std::{env, fs};

// main func
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
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

// struct
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = if env::var("CASE_INSENSITIVE").is_err() {
            args.find(|s| s == &String::from("-i")).is_none()
        } else {
            false
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// impl
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn two_results() {
        let query: &str = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Rust.";

        assert_eq!(vec!["Rust:", "Rust."], search(query, contents));
    }

    #[test]
    fn no_results() {
        let query: &str = "Ruwst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Rust.";

        assert!(search(query, contents).is_empty());
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
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
