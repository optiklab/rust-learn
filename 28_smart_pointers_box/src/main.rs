// https://doc.rust-lang.org/stable/book/ch15-01-box.html
use std::ops::Deref;

////////////////////////////////////////////////////////////////////////////////
/////////////////// Defining custom smart pointer type MyBox<T> ////////////////
////////////////////////////////////////////////////////////////////////////////
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// Using the dereference operator on a MyBox<T> would be impossible without it:
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////// Enabling Recursive Types with Boxes /////////////////////
////////////////////////////////////////////////////////////////////////////////
// (1, (2, (3, Nil)))
#[derive(Clone)]
enum List {
    Cons(i32, Box<List>), // Cons(i32, List) will not compile
    Nil,
}

impl Iterator for List {
    type Item = i32; // В итераторе мы устанавливаем связанный тип Item равным u32, 
                     // то есть итератор будет возвращать значения типа u32.

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Cons(head, tail) => {
                let value = *head;
                *self = *tail.clone(); // Move the tail into self
                Some(value)
            }
            Nil => None,
        }
    }
}

use crate::List::{Cons, Nil};

// Recoursively printing the list
fn print_list(list: &List) {
    match list {
        Cons(head, tail) => {
            print!("{head} ");
            print_list(tail);
        }
        Nil => println!("Nil"),
    }
}

////////////////////////////////////////////////////////////////////////////////
// // https://doc.rust-lang.org/stable/book/ch15-02-deref.html
////////////////////////////////////////////////////////////////////////////////
//////////// A Custom smart pointer that implements the Drop trait /////////////
// ////////////////////// with custom cleanup logic ////////////////////////////
////////////////////////////////////////////////////////////////////////////////
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn useCustomSmartPointerWithDropTrait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn dropCustomSmartPointerBeforeOutOfScope() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // Calling std::mem::drop to explicitly drop a value before it goes out of scope
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

////////////////////////////////////////////////////////////////////////////////
///////////////////////////// Deref coercion example ///////////////////////////
////////////////////////////////////////////////////////////////////////////////
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // Using the dereference operator to follow a reference to an i32 value
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using the dereference operator on a Box<i32>
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Attempting to use MyBox<T> in the same way we used references and Box<T>    
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // behind the scenes Rust actually runs: *(y.deref())
    
    // Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // The code we would have to write if Rust didn’t have deref coercion:
    // hello(&(*m)[..]);

    /////////////////////// Enabling Recursive Types with Boxes ////////////////
    // (1, (2, (3, Nil)))
    // Do not compiles: let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Print list recoursively:");
    print_list(&list);
    // Print list in for loop
    println!("Print list in for loop:");
    for i in list {
        print!("{i} ")
    }
    println!("Done!");

    // https://doc.rust-lang.org/stable/book/ch15-02-deref.html
    useCustomSmartPointerWithDropTrait();
    dropCustomSmartPointerBeforeOutOfScope();
}