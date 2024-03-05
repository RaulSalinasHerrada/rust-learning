use std::{env, fs}; //filesystem, to read
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    

    for line in results {
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
    pub fn new(args: &[String]) -> Result<Self, &str> {

        if args.len() < 3{
            return Err("Not enough arguments!");
        }
        let query = String::from(&args[1]); // 0 is bin path
        let filename = String::from(&args[2]);
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        println!("searching: {}", query);
        println!("in file: {}", filename);
        println!("Case sensitive: {}", case_sensitive);

        Ok(Config {query, filename, case_sensitive})
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

pub fn search_insensitive<'a>(
    query: &str, contents: &'a str
    ) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line  in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape
";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
} 