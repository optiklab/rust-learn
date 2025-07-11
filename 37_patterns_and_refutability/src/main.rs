// Patterns come in two forms: refutable and irrefutable. 
// - Patterns that will match for any possible value passed are irrefutable. 
//   An example would be x in the statement let x = 5; because x matches anything 
//   and therefore cannot fail to match.
// - Patterns that can fail to match for some possible value are refutable. 
//   An example would be Some(x) in the expression if let Some(x) = a_value 
//   because if the value in the a_value variable is None rather than Some, 
//   the Some(x) pattern will not match.

fn main() {
    // let Some(x) = some_option_value; // error[E0005]: pattern `None` not covered

    // If we have a refutable pattern where an irrefutable pattern is needed,
    // we can fix it by changing the code that uses the pattern: instead of 
    // using let, we can use if let. Then if the pattern doesn’t match, 
    // the code will just skip the code in the curly brackets.
    let Some(x) = some_option_value else {
        return;
    };

    // However, if we give 'if let' an irrefutable pattern (a pattern that 
    // will always match):
    //let x = 5 else { // warning: irrefutable `let...else` pattern
                       // pattern will always match - `else` clause is useless
    //    return;
    //};

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let xx = Some(5);
    let y = 10;
    match xx {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // A match expression 
                                // with an arm that introduces a new variable 
                                // which shadows an existing variable y
        _ => println!("Default case, x = {xx:?}"),
    }
    println!("at the end: x = {xx:?}, y = {y}");

    // Multiple patterns via |
    let xxx = 1;
    match xxx {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    let xxxx = 5;
    match xxxx {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Destructuring Structs
    destruct_into_variables();
    destruct_using_struct_field_shorthand();

    // Destructuring and matching literal values in one pattern
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    destruct_enum();
    destruct_nested_enum();

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);
}

////////////////////////////////////////////////////////////////////////////////

struct Point {
    x: i32,
    y: i32,
}

fn destruct_into_variables() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn destruct_using_struct_field_shorthand() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

////////////////////////////////////////////////////////////////////////////////

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destruct_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

enum Color1 {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color1),
}

fn destruct_nested_enum() {
    let msg = Message1::ChangeColor(Color1::Hsv(0, 160, 255));

    match msg {
        Message1::ChangeColor(Color1::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message1::ChangeColor(Color1::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}

////////////////////////////Ignoring Values in a Pattern ///////////////////////

// An Entire Value with _
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");

    // Starting a variable name with an underscore to avoid 
    // getting unused variable warnings:    
    let _x = 5;
    let y = 10;

    // Don't miss that the syntax _s still binds the value to the variable,
    // whereas _ doesn’t bind at all. 
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");
}

// Parts of a Value with a Nested _
fn ignore_parts_of_value_with_nested() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {

        // Using an underscore within patterns that match Some variants 
        // when we don’t need to use the value inside the Some

        (Some(_), Some(_)) => { // test for the case when setting_value and 
                                // new_setting_value are the Some variant or not!
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            // if either setting_value or new_setting_value is None in the second arm, 
            // we want to allow new_setting_value to become setting_value.
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    // This code will print 'Can't overwrite ...' and then 'setting is Some(5)'.
}

// _ pattern to match and ignore specific parts of array
fn ignore_some_parts() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
}

// Dots pattern to match rest values of array
fn remaining_values_pattern() {

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {x}"),
    } 
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// Dots pattern to match specific parts of array
fn matching_only_some_parts() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // But it must be unambiguous... so this will fail:
    // match numbers {
    //    (.., second, ..) => {
    //        println!("Some numbers: {second}")
    //    },
    // }
}

fn match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    // Using a match guard to test for equality with an outer variable
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    // Combining multiple patterns with a match guard
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
///////////////////////////// At @ bindings ///////////////////////////////////
enum Message2 {
    Hello { id: i32 },
}

fn at_bindings() {
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {id}"),
    }
}