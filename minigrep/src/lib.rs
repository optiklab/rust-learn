use std::error::Error;
use std::fs;

pub struct Config {
    pub search_query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    /////////////////////////////// OLD BOOK ///////////////////////////////////
    // Interpretation of OLD version of book... it is POSSIBLE to do this way, 
    // but the latest book suggests to use the build() method instead.

    /*
    // Less efficient
    // https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
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
    */

    // More efficient
    // https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let search_query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query for search"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename"),
        };
        let ignore_case = std::env::var("IGNORE_CASE").is_ok(); 
        Ok(Config { search_query, file_path, ignore_case })
    }
    ///////////////////////////// End of OLD BOOK //////////////////////////////
    /*
    // Less efficient
    // https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
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
    */
    // More efficient!
    // https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let search_query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            search_query,
            file_path,
            ignore_case,
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

/*
// Old style
// https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
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
*/

// New way using filters and iterators
// https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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