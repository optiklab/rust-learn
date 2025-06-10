use std::error::Error;
use std::fs;

pub struct Config {
    pub search_query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        println!("Accpted: {:?}", args);
        dbg!(&args);

        if args.len() < 3 {        
            return Err("Not enough arguments provided. Expected at least 2 arguments.");
        }
        
        Ok(Config {
            search_query: args[1].clone(), // In Chapter 13, you’ll learn how to use more efficient methods than clone(). 
            file_path: args[2].clone(), // https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(&config.file_path)?;

    println!("With text:\n{contents}");

    // Here you can add the logic to search for config.search_query in contents
    // For example, you can use regex or simple string matching.
    // This is just a placeholder for demonstration.
    if contents.contains(&config.search_query) {
        println!("Found '{}' in the file!", config.search_query);
    } else {
        println!("'{}' not found in the file.", config.search_query);
    }
    
    Ok(())
}