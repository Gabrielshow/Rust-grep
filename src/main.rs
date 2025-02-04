extern crate minigrep;

use std::env;
// use std::fs::File;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let (query, filenme) = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}" , err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {} \n", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
    // println!("{:?}", args);
}

