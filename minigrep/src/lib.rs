use std::error::Error;
use std::fs;

pub struct Config {
    pub search_query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    // Experiment
        // JUST MY OWN INTERPRETATION OF THE BOOK... it is POSSIBLE to do this way, but the book suggests to use the build() method instead.
        pub fn new(&mut self, args: &[String]) -> Result<&Config, &'static str> {

            println!("Accpted: {:?}", args);
            dbg!(&args);

            if args.len() < 3 {        
                return Err("Not enough arguments provided. Expected at least 2 arguments.");
            }
            self.search_query = args[1].clone(); // In Chapter 13, you’ll learn how to use more efficient methods than clone(). 
            self.file_path = args[2].clone(); // https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html
            self.ignore_case = std::env::var("IGNORE_CASE").is_ok();  // If the environment variable is set, ignore case
                // $Env:IGNORE_CASE=1; cargo run -- to poem.txt
                // Remove-Item Env:IGNORE_CASE
            
            Ok(self)
        }

    // End of experiment

    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        println!("Accpted: {:?}", args);
        dbg!(&args);

        if args.len() < 3 {        
            return Err("Not enough arguments provided. Expected at least 2 arguments.");
        }
        
        Ok(Config {
            search_query: args[1].clone(), // In Chapter 13, you’ll learn how to use more efficient methods than clone(). 
            file_path: args[2].clone(), // https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html
            ignore_case: std::env::var("IGNORE_CASE").is_ok()  // If the environment variable is set, ignore case
                // $Env:IGNORE_CASE=1; cargo run -- to poem.txt
                // Remove-Item Env:IGNORE_CASE
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(&config.file_path)?;

    println!("With text:\n{contents}");

    let result;
    if config.ignore_case {
        result = search_ignore_case(&config.search_query, &contents);
    } else {
        result = search(&config.search_query, &contents);
    }

    if result.len() > 0 {
        println!("Found '{}' in the file:", config.search_query);
    } else {
        println!("'{}' not found in the file!", config.search_query);
    }

    for line in result {
        println!("  {line}");
    }
    
    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results

    // vec![] // Empty vector
}

fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
Ductiloscopy is not a programming language.
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

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
            search_ignore_case(query, contents)
        );
    }
}