use std::env;
use std::process;

use minigrep::Config;

// Finds 2 lines with case sensitive search of "to" in poem.txt:
// cargo run -- to poem.txt

// If the environment variable is set, ignore case and find 4 lines:
// $Env:IGNORE_CASE=1; cargo run -- to poem.txt
// Remove-Item Env:IGNORE_CASE

fn main() {

    let args: Vec<String> = env::args() // std::env::args will panic 
        // if any argument contains invalid Unicode. If your program needs to accept 
        // arguments containing invalid Unicode, use std::env::args_os instead. 
        // That function returns an iterator that produces OsString values 
        // instead of String values. We’ve chosen to use std::env::args here 
        // for simplicity because OsString values differ per platform and 
        // are more complex to work with than String values.
        .collect(); // collect function may create many kinds of collections, 
                // so we explicitly annotate the type we expect: Vec<String>

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        println!("Usage: {} <search_query> <file.ext>", args[0]);
        process::exit(1); // Exit with error code 1
    });

    println!("Searching for '{}'", config.search_query);
    println!("In file '{}'", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1); // Exit with error code 1
    }
}

/*
fn main() {

    let args: Vec<String> = env::args() // std::env::args will panic 
    // if any argument contains invalid Unicode. If your program needs to accept 
    // arguments containing invalid Unicode, use std::env::args_os instead. 
    // That function returns an iterator that produces OsString values 
    // instead of String values. We’ve chosen to use std::env::args here 
    // for simplicity because OsString values differ per platform and 
    // are more complex to work with than String values.
    .collect(); // collect function may create many kinds of collections, 
                // so we explicitly annotate the type we expect: Vec<String>

    //let (search_query, file_path) = parse_inputs(&args);
    //let config = parse_inputs(&args); // panics
    //let config = Config::new(&args); // panics
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        println!("Usage: {} <search_query> <file.ext>", args[0]);
        process::exit(1); // Exit with error code 1
    });

    println!("Searching for '{}'", config.search_query);
    println!("In file '{}'", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1); // Exit with error code 1
    }
}


struct Config {
    search_query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

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
    /*
    fn new(args: &[String]) -> Config {

        println!("Accpted: {:?}", args);
        dbg!(&args);

        if args.len() < 3 {        
            println!("Usage: {} <search_query> <file.ext>", args[0]);
            panic!("Not enough arguments provided. Expected at least 2 arguments.");
        }
        
        Config {
            search_query: args[1].clone(), // In Chapter 13, you’ll learn how to use more efficient methods than clone(). 
            file_path: args[2].clone(), // https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html
        }
    }*/
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

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
*/


/*
fn parse_inputs(args: &[String]) -> Config { // (&str, &str) {

    println!("Accpted: {:?}", args);
    dbg!(&args);

    if args.len() < 3 {        
        println!("Usage: {} <search_query> <file.ext>", args[0]);
        panic!("Not enough arguments provided. Expected at least 2 arguments.");
    }
    
    Config {
        search_query: args[1].clone(), // In Chapter 13, you’ll learn how to use more efficient methods than clone(). 
        file_path: args[2].clone(), // https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html
    }
    // (&args[1], &args[2])
}
*/

/*
// Initial version
fn main() {

    let args: Vec<String> = env::args() // std::env::args will panic 
    // if any argument contains invalid Unicode. If your program needs to accept 
    // arguments containing invalid Unicode, use std::env::args_os instead. 
    // That function returns an iterator that produces OsString values 
    // instead of String values. We’ve chosen to use std::env::args here 
    // for simplicity because OsString values differ per platform and 
    // are more complex to work with than String values.
    .collect(); // collect function may create many kinds of collections, 
                // so we explicitly annotate the type we expect: Vec<String>

    if args.len() < 2 {
        println!("Usage: {} <search_query> <file.ext>", args[0]);
        return;
    }

    println!("Accpted: {:?}", args);

    dbg!(&args);

    let search_query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{search_query}'");
    println!("In file '{file_path}'");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
*/