fn main() {
    let mut s: char = 'a'; // RUSTC:    mov     dword ptr [rsp - 4], 97

    let str1 = "Microsoft Edge"; // Is of type &str (not String!)

    println!("A String is always {:?} bytes. It is Sized.",
        std::mem::size_of::<String>()); // Prints the size in bytes of a type.
        // 24 - it is composed of
            // a pointer (8 bytes on 64bit systems) to the heap, 
            // a 8 byte length,
            // and a 8 byte capacity.

    println!("&str can be anything. '서태지' is {:?} bytes. It is not Sized.", 
        std::mem::size_of_val("서태지")); // gives you the size in bytes of a variable
    println!("'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", 
        std::mem::size_of_val("Adrian Fahrenheit Țepeș"));

    let mut s1 = String::new();
    println!("s1 len: {}", s1.len()); // Prints 0
    
    s1 = "Anton".to_string();
    println!("s1 len: {}", s1.len());

    s1.push_str(" Yarkov"); // OR +=
    s1.push('!'); // one byte char

    println!("s1: {}", s1);

    let str2 = &s1[0..5];
    println!("Slice of s1: {str2}");

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

    let str1 = String::from("Hello, ");
    let str2 = String::from("world!");
    let str3 = str1 + &str2; // note s1 has been moved here and can no longer be used

    let tic1 = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tictactoe = tic1 + "-" + &tac + "-" + &toe;

    let tic2 = String::from("tic"); // We cannot use tic1 anymore, but we can use tic2.
    let tictactoe_formatted = format!("{tic2}-{tac}-{toe}"); // format!() does not move, it copies

    dont_move_my_char(s); // OK. It's copied!

    conventional_example_of_moves();

    return_cortage_by_moves();
    return_cortage_by_ref();

    slicing();
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
    let len = calculate_length_ref(&s1); // s1 passed as a reference - not move.
                                   // but value is immutable - can't change!
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}  // Nothing happens with s, because it was not owned.

fn slicing() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("Slice &hello[0..4] of '{hello}': {s}"); // Prints "Зд"
    // println!("{}", &hello[0]); // COMPILE ERROR! UTF-8 chars are not 1 byte long.
    println!("Slice &hello[0..2] of '{hello}': {}", &hello[0..2]);
    // println!("{}", &hello[0..1]); // COMPILE ERROR! UTF-8 chars are not 1 byte long.
    println!("Length of '{hello}': {}", hello.len());

    println!("Print '{hello}' char by char: {} chars in total", hello.chars().count());
    for c in hello.chars() {
        println!("{c}");
    }

    println!("Print '{hello}' byte by byte: {} bytes in total", hello.as_bytes().len());
    let mut bytesString = String::new();
    for b in hello.bytes() {
        bytesString.push_str(&format!("{b}, "));
    }
    bytesString.pop(); // Remove the last comma
    bytesString.pop(); // Remove the last space
    println!("{bytesString}");

    println!("Length of '{s}': {}", s.len());

    let hindi = "नमस्ते";
    println!("'{hindi}': {} chars in total", hindi.chars().count());
    
    for c in hindi.chars() {
        println!("{c}");
    }

    println!("'{hindi}': {} bytes in total", hindi.bytes().count());
}