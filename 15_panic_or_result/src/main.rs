//use std::fs;
//use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;
use rand::Rng;
use std::cmp::Ordering;

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
// Shorter version using the ? operator
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Even more shorter
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Actually... standard function in fs namespace doing that already:
fn read_username_from_file_standard() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

// Using the ? operator on an Option<T>(Some, None) value (not only for Result<T, E>):
fn last_char_of_first_line(text: &str) -> Option<char> {
    // next() might return None
    // or might return Some(line)
    text.lines().next()?.chars().last()
}

/*
use std::error::Error;
use std::fs::File;

// Also possible to use the ? operator in main function, 
// but it requires the main function to return a Result type
// where the ERROR can be ANY TYPE that implements the Error trait.
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/

fn main() {
    let greeting_file_result = File::open("hello.txt");

    /*// Simple
    let greeting_file1 = match greeting_file_result {
        Ok(file1) => file1,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    */

    let greeting_file2 = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    let greeting_file3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // Shortcuts:

    // unwrap already doing that for us
    let greeting_file4 = File::open("hello.txt").unwrap();
    
    // expect is similar to unwrap, but allows you to provide a custom panic message.
    let greeting_file5 = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    //let greeting_file = File::open("hello.txt")?; // Will not compile
    
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    guessing_game();
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn guessing_game() {
    let secret_number: i32 = rand::rng().random_range(1..11);

    loop {
        println!("Guess a number:");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess) // There is also io::Result returned. Enum with Ok and Err values.
            .expect("Failed to read line"); // An error of not handled return if you don't call expect().

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YESSS! Bye!");
                return;
            }
        }
    }
}   