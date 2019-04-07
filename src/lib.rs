use std::fs;
use std::error::Error;
use std::str::FromStr;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments");
        }
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get filename")
        };
        let case_sensitive = match args.next() {
            Some(arg) => bool::from_str(&arg).unwrap_or_else(|e| panic!("case sensitive option is not a bool")),
            None => return Err("Didn't get case sensitive option")
        };
        Ok(Config { query, filename, case_sensitive })
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase() as &str)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}
