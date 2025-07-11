// https://doc.rust-lang.org/stable/book/ch15-04-rc.html

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }
    //
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // <-- problem! Not compiles!

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Ref count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Ref count after creating b = {}", Rc::strong_count(&a));

    let c = Cons(4, Rc::clone(&a)); // <-- no problem!
    println!("Ref count after creating c = {}", Rc::strong_count(&a));

    {
        let d = Cons(5, Rc::clone(&a)); // <-- no problem!
        println!("Ref count after creating d = {}", Rc::strong_count(&a));
    }

    println!("Ref count after d goes out of scope = {}", Rc::strong_count(&a));
}
