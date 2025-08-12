//use std::fs;
//use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;
use std::cmp::Ordering;
use std::fmt; // For Error + Debug + Display trait.
use std::error::Error; // For Error trait.
use thiserror::Error;

fn main() {
}

// We've seen how to implement the `Error` trait "manually" for a custom error type.
// Imagine that you have to do this for most error types in your codebase. That's a lot of boilerplate, isn't it?
// We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
// a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.

// TODO: Implement the `Error` trait for `TicketNewError` using `thiserror`.
//   We've changed the enum variants to be more specific, thus removing the need for storing
//   a `String` field into each variant.
//   You'll also have to add `thiserror` as a dependency in the `Cargo.toml` file.

#[derive(Error, Debug)]
enum TicketNewError2 {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket2 {
    title: String,
    description: String,
    status: Status2,
}

#[derive(Debug, PartialEq, Clone)]
enum Status2 {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket2 {
    pub fn new(
        title: String,
        description: String,
        status: Status2,
    ) -> Result<Ticket2, TicketNewError2> {
        if title.is_empty() {
            return Err(TicketNewError2::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError2::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError2::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError2::DescriptionTooLong);
        }

        Ok(Ticket2 {
            title,
            description,
            status,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
//////////////////////////////    End of `thiserror`    ////////////////////////
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////   Into() & From()  //////////////////////////
////////////////////////////////////////////////////////////////////////////////

// Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
// The parsing should be case-insensitive.

#[derive(Error, Debug)]
enum TicketError3 {
    #[error("Ticked status is undefined.")]
    SomeError
}

#[derive(Debug, PartialEq, Clone)]
enum Status3 {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status3 {
    type Error = TicketError3; 
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Status3::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Status3 {
    type Error = TicketError3; 
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status3::ToDo),
            "inprogress" => Ok(Status3::InProgress),
            "done" => Ok(Status3::Done),
            _ => Err(TicketError3::SomeError)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////  End of Into() & From()  //////////////////////////
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////
////////////////////// Error source and propagation ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

use crate::status::Status;
use crate::status::ParseStatusError;

// We've seen how to declare modules in one of the earliest exercises, but
// we haven't seen how to extract them into separate files.
// Let's fix that now!
//
// In the simplest case, when the extracted module is a single file, it is enough to
// create a new file with the same name as the module and move the module content there.
// The module file should be placed in the same directory as the file that declares the module.
// In this case, `src/lib.rs`, thus `status.rs` should be placed in the `src` directory.
mod status;

// TODO: Add a new error variant to `TicketNewError` for when the status string is invalid.
//   When calling `source` on an error of that variant, it should return a `ParseStatusError` rather than `None`.

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{0}")]
    StatusInvalid(#[from] ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // Parse the status string into a `Status` enum.
        let valid_status = Status::try_from(status)?;

        Ok(Ticket {
            title,
            description,
            status: valid_status,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
/////////// End of Error source and propagation
////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use super::*;

    // Helpers
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
    // End of Helpers

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// this error tests ////////////////////////////
////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn title2_cannot_be_empty() {
        let err = Ticket2::new("".into(), valid_description(), Status2::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be empty");
    }

    #[test]
    fn description2_cannot_be_empty() {
        let err = Ticket2::new(valid_title(), "".into(), Status2::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Description cannot be empty");
    }

    #[test]
    fn title2_cannot_be_longer_than_fifty_chars() {
        let err = Ticket2::new(overly_long_title(), valid_description(), Status2::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description2_cannot_be_too_long() {
        let err = Ticket2::new(valid_title(), overly_long_description(), Status2::ToDo).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Description cannot be longer than 500 bytes"
        );
    }

////////////////////////////////////////////////////////////////////////////////
/////////// TryFrom tests
////////////////////////////////////////////////////////////////////////////////
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status3::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status3::ToDo);

        let status = Status3::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status3::InProgress);

        let status = Status3::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status3::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status3::try_from("todo").unwrap();
        assert_eq!(status, Status3::ToDo);

        let status = Status3::try_from("inprogress").unwrap();
        assert_eq!(status, Status3::InProgress);

        let status = Status3::try_from("done").unwrap();
        assert_eq!(status, Status3::Done);
    }

////////////////////////////////////////////////////////////////////////////////
/////////// Tests for Error source and propagation
////////////////////////////////////////////////////////////////////////////////

    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}




