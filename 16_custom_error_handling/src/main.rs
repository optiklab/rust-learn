//use std::fs;
//use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;
use std::cmp::Ordering;
use std::fmt; // For Error + Debug + Display trait.
use std::error::Error; // For Error trait.

///////////////////////////////////////////////////////////////////////////////
// Use`Ticket::new` method to return a `Result` instead of panicking.
////////////////// Use `String` as the error type. ////////////////////////////

fn main() {
}

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

// Implementing `Debug`, `Display` and `Error` for the `TicketNewError` enum.
//  When implementing `Display`, you may want to use the `write!` macro from Rust's standard library.
//  The docs for the `std::fmt` module are a good place to start and look for examples:
//  https://doc.rust-lang.org/std/fmt/index.html#write
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

// `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
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

