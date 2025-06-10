use std::env;
use std::fs;

// cargo run -- the poem.txt
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
