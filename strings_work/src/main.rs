fn main() {
    let mut s: char = 'a'; // RUSTC:    mov     dword ptr [rsp - 4], 97

    let mut s1 = String::new();
    s1 = "Anton".to_string();

    s1.push_str(" Yarkov"); // OR +=

    println!("s1: {}", s1);

    let mut s2 = String::from("Hello");
    s2 += " World!"; // OR s2.push_str(...)
    println!("s2: {}", s2);

    let s3 = s2; // MOVE  https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
    println!("s3: {} (s2 moved to s3)", s3);
    println!("(s2 freed)");
    //println!("{}", s2); // COMPILE ERROR https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

    let s4 = s3.clone(); // CLONE  https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
    println!("s4: {} (s3 cloned to s4)", s4);
    println!("s3: {} (s3 still alive)", s3);

    copy_or_not();
    move_my_string(s4);
    //println!("{}", s4); // COMPILE ERROR! It's moved. https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

    dont_move_my_char(s); // OK. It's copied!

    conventional_example_of_moves();

    return_cortage_by_moves();
    return_cortage_by_ref();
}

fn copy_or_not() {
    let tup1: (i32, i32) = (1, 2);
    let (x, y) = tup1; // COPY https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

    let tup2: (i32, String) = (1, "2".to_string());
    let (x, y) = tup2; // MOVE https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
}

fn move_my_string(str: String) {

    println!("{str}");
}

fn dont_move_my_char(symbol: char) {

    println!("{}", symbol);
}

fn conventional_example_of_moves() {
    let s1 = gives_ownership();        // moves its return value into s1
    println!("{}", s1);

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved to takes_and_gives_back, 
                                       // which also moves its return back to s3
    println!("{}", s3);
    //println!("{}", s2); // COMPILE ERROR!
} // s3 goes out of scope and is dropped. s2 was moved, so nothing happens. 
  // s1 goes out of scope and is dropped.

// Moves its return value to the caller
fn gives_ownership() -> String {       

    let some_string = String::from("yours"); // some_string comes into scope

    some_string  // some_string is returned and moves out to the caller
}

// Takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string 
    // a_string is returned and moves out to the calling function
}




fn return_cortage_by_moves() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}



fn return_cortage_by_ref() {
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}