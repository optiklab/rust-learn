//use std::fs;
//use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;
use rand::Rng;
use std::cmp::Ordering;
use std::fmt; // For Error + Debug + Display trait.
use std::error::Error; // For Error trait.

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

///////////////////////////////////////////////////////////////////////////////
// Use`Ticket::new` method to return a `Result` instead of panicking.
////////////////// Use `String` as the error type. ////////////////////////////

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

#[derive(Debug)]
enum TicketNewError {
    TitleError { description: String },
    DescriptionError { description: String }
}

// EVERY ERROR ENUM HAVE TO IMLPEMENT ERROR TRAIT + DEBUG + DISPLAY.
impl fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketNewError::TitleError { description } => write!(f, "{}", description),
            TicketNewError::DescriptionError { description } => write!(f, "{}", description)
        }
    }
}

impl Error for TicketNewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            TicketNewError::TitleError { description: _ } => Some(self),
            TicketNewError::DescriptionError { description: _ } => Some(self)
        }
    }
}

impl Ticket {
    ///////////////////// Use `String` as the error type. //////////////////////
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    ////////////////// Use `enum` as the error type. ///////////////////////////
    pub fn new_alt(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError { description: "Title cannot be empty".to_string() });
        }

        if title.len() > 50 {
            return Err(TicketNewError::TitleError { description: "Title cannot be longer than 50 bytes".to_string() });
        }

        if description.is_empty() {
            return Err(TicketNewError::DescriptionError { description: "Description cannot be empty".to_string() });
        }

        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError { description: "Description cannot be longer than 500 bytes".to_string() });
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}


fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    if title.is_empty() {
        panic!("Title cannot be empty");
    }
    if title.len() > 50 {
        panic!("Title cannot be longer than 50 bytes");
    }

    let mut desc = description;
    if desc.is_empty() || desc.len() > 500 {
        desc = String::from("Description not provided");
    }

    Ticket {
        title,
        description: desc,
        status,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ticket_title_cannot_be_empty() {
        let error = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be empty");
    }

    #[test]
    fn new_alt_ticket_title_cannot_be_empty() {
        let error: TicketNewError = Ticket::new_alt("".into(), valid_description(), Status::ToDo).unwrap_err();
        match error {
            TicketNewError::TitleError { description } => assert_eq!(description, "Title cannot be empty"),
            TicketNewError::DescriptionError { description } => assert!(false)
        }
    }

    #[test]
    fn new_ticket_description_cannot_be_empty() {
        let error = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be empty");
    }

    #[test]
    fn new_alt_ticket_description_cannot_be_empty() {
        let error: TicketNewError = Ticket::new_alt(valid_title(), "".into(), Status::ToDo).unwrap_err();
        match error {
            TicketNewError::TitleError { description } => assert!(false),
            TicketNewError::DescriptionError { description } => assert_eq!(description, "Description cannot be empty")
        }
    }

    #[test]
    fn new_ticket_title_cannot_be_longer_than_fifty_chars() {
        let error =
            Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn new_ticket_description_cannot_be_longer_than_500_chars() {
        let error =
            Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(error, "Description cannot be longer than 500 bytes");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn easy_ticket_title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn easy_ticket_template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn easy_ticket_title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn easy_ticket_template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
    
    pub fn overly_long_description() -> String {
        "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".into()
    }

    pub fn overly_long_title() -> String {
        "A title that's definitely longer than what should be allowed in a development ticket".into()
    }

    pub fn valid_title() -> String {
        "A title".into()
    }

    pub fn valid_description() -> String {
        "A description".into()
    }
}
