use std::{error::Error, fs};
use super::grep::{search, search_case_insensitive};

pub fn read_poem(file_path: &str, query: &str) {
    println!("Reading {}", file_path);

    let contents =
        fs::read_to_string(file_path).unwrap_or_else(|e| panic!("Could not read '{file_path}': {e}"));

    for line in contents.lines().filter(|l| l.contains(query)) {
        println!("{line}");
    }
}

pub struct Config {
    query: String,
    file_path: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments, less than 3!");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Config { query, file_path };
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Argument length is less than 3!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
    }
}

pub fn original_parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    return Config {query, file_path};
}

pub fn run_config(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    return Ok(());
}

#[cfg(test)]
mod sample_search_test {
    use super::*;

    #[test]
    pub fn search_test() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    
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