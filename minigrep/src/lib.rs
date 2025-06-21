//! # MiniGrep
//!
//! `minigrep` is a custom version of the classic command line search tool 
//! grep (globally search a regular expression and print) that 
//! searches a specified file for a specified string. 
//! To do so, it takes a file path and a string as its arguments. 
//! Then it reads the file, finds lines in that file, and prints those lines.

pub mod my_random_module {

    pub struct User {
        pub name: String
    }
}

pub mod minigrep_module {

    use std::error::Error;
    use std::fs;

    use super::*;

    /// A configuration parameters for grep search. 
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

        /// DEPRECATED.
        /// Efficiently uses Iterators to read arguments and instantiate
        /// GREP configuration. Though, using old patterns.
        /// Example taken from https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html
        ///
        /// # Examples
        ///
        /// ```
        /// let mut args: Vec<String> = Vec::new();
        /// args.push("--".into());
        /// args.push("to".into());
        /// args.push("poem.txt".into());
        ///
        /// let config = minigrep::minigrep_module::Config::build(args.into_iter()).unwrap(); // I am using minigrep:: to resolve
        ///
        /// assert_eq!("poem.txt", config.file_path);
        /// assert_eq!("to", config.search_query);
        /// ```
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

        /// Efficiently uses Iterators to read arguments and instantiate
        /// GREP configuration.
        /// Example taken from https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html
        ///
        /// # Examples
        ///
        /// ```
        /// let mut args: Vec<String> = Vec::new();
        /// args.push("--".into());
        /// args.push("to".into());
        /// args.push("poem.txt".into());
        ///
        /// let config = minigrep::minigrep_module::Config::build(args.into_iter()).unwrap(); // I am using minigrep:: to resolve
        ///
        /// assert_eq!("poem.txt", config.file_path);
        /// assert_eq!("to", config.search_query);
        /// ```
        /// 
        /// #Errors
        /// "Didn't get a query string" in case search query is not specified.
        /// "Didn't get a file path" in case file path is not specified.
        ///
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

    /// Executes search of the strings in the specified file by specified pattern.
    /// By default, search is case sensitive. Set environment variable IGNORE_CASE = 1 
    /// to execute case insensitive (ignore case) search.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut args: Vec<String> = Vec::new();
    /// args.push("--".into());
    /// args.push("to".into());
    /// args.push("poem.txt".into());
    ///
    /// let config = minigrep::minigrep_module::Config::build(args.into_iter()).unwrap(); // I am using minigrep:: to resolve
    /// // Alternative:
    /// //let config = Config::build(env::args()).unwrap();
    /// if let Err(e) = minigrep::minigrep_module::run(config) {
    ///     eprintln!("Application error: {e}");
    /// }
    /// ```
    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

        let contents = fs::read_to_string(&config.file_path)?;

        println!("With text:\n{contents}");

        let result;
        if config.ignore_case {
            result = search_utilities::search_ignore_case(&config.search_query, &contents);
        } else {
            result = search_utilities::search(&config.search_query, &contents);
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
}

mod search_utilities {

    // New way using filters and iterators
    // https://doc.rust-lang.org/stable/book/ch13-03-improving-our-io-project.html
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        let query = query.to_lowercase();
        
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
        
        results
    }
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

        assert_eq!(vec!["safe, fast, productive."], search_utilities::search(query, contents));
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
            search_utilities::search_ignore_case(query, contents)
        );
    }
}