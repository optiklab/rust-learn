/*
enum Option<T> {
    None,
    Some(T),
}

Included in prelude (no need for namespace).
None and Some can be used without Option::
*/

fn main() {
    let some_number = Some(5);
    //let some_other_number = Some(5);
    // let some = some_number + some_other_number; // error[E0369]: cannot add `Option<{integer}>` to `Option<{integer}>`

    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y; // error[E0277]: cannot add `Option<i8>` to `i8`

    let z = match y {
        None => None,
        Some(i) => Some(x + i), // aka = x + y
    };
}

//////////// Excersizes ////////////// Enum with value /////////////////////////
// TODO: Implement `Ticket::assigned_to`.
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

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn assigned_to_alt(&self) -> Option<&String> {
        let result = match &self.status {
            Status::InProgress { assigned_to } => Some(assigned_to),
            Status::Done | Status::ToDo => None
        };

        result
    }
}