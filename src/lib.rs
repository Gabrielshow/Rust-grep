use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // println!("With text: {}\n", contents);
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String, //first query parameter
    pub filename: String, //name of file
    pub case_sensitive: bool, // case sensitivity
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config{ query, filename, case_sensitive })
    }  
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
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
// std::env::args will panic if any argumnet contains invalid Unicode.
// if your program neesd to accept arguments contianing invalid Unicode, 
// use std::env::args_os instead. That function returns an iterator that produces
// OsString values instead of String values. We've chosen to use std::env::args here for simplicity,
// because OsString values differ per platform and are more complex to work with than string values.
#[cfg(test)]
mod test {
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
    fn case_sensitive() {
        let query= "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust: 
safe, fast , productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust", "Trust me."], search_case_insensitive(query, contents));
    }
}