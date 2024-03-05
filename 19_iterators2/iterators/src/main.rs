use std::env::{self, Args};
use std::process;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config{
    pub fn new<'a>(mut args: Args) -> Result<Config, &'static str> {

        args.next(); //path of program

        if args.len() < 3 {return Err("not enough arguments")}

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE")
                                    .is_err();

        Ok(
            Config{query, filename, case_sensitive}
        )
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn main() {
    let mut args  = env::args();

    let config = Config::new(args).unwrap_or_else(
    |err| {
        eprintln!("Problem parting arguments {}", err);
        process::exit(1);
    }
    );

}
