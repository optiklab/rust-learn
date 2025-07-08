use trait_as_an_abstraction::gui::{Button, Screen};

// The advantage of using trait objects and Rust’s type system to write code 
// similar to code using duck typing is that we never have to check whether 
// a value implements a particular method at runtime or worry about getting 
// errors if a value doesn’t implement a method but we call it anyway. 
// Rust won’t compile our code if the values don’t implement the traits 
// that the trait objects need.
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    // Attempting to use a type that doesn’t implement the trait object’s trait,
    // will get this error because String doesn’t implement the Draw trait.
    //let screen = Screen {
    //    components: vec![Box::new(String::from("Hi"))],
    //};

    screen.run();
}

//////////////////// Imagine this is ANOTHER folder and MODULE /////////////////
///////// Someone using our library decides to implement a SelectBox ///////////
use trait_as_an_abstraction::gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
////////////////////////////////////////////////////////////////////////////////