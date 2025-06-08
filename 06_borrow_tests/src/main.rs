fn main() {
    return_cortage_by_moves();
    return_cortage_by_ref();
    mutable_reference_test();
    two_mutable_references_test();

    slices_test();
    slices_test2();
    slices_test3();
}

fn slices_test() {

    println!("### Slice it to hell:");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    //s.clear(); // COMPILE ERROR!
    println!("{hello}.");
    println!("{world}.");
}

fn slices_test2() {
    println!("### Slice it to hell 2:");
    let microsoft_edge_string = "Microsoft Edge!".to_string();
    let word = first_word(&microsoft_edge_string);
    println!("{word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slices_test3() {
    println!("### Slice it to hell 3:");
    let microsoft_edge_str = "Microsoft Edge!"; // &str
    let microsoft_edge_string = String::from("Microsoft Edge!"); // String
    println!("{}", better_first_word(&microsoft_edge_str));
    println!("{}", better_first_word(&microsoft_edge_string[..]));
}

fn better_first_word(s: &str) -> &str { // References will work for both 
                                        // &str and String. Better!
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn return_cortage_by_moves() {
    println!("### return_cortage_by_moves:");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // s1 MOVED,
                                          // then cortage s2, len moved back.
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}



fn return_cortage_by_ref() {
    println!("### return_cortage_by_ref:");
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1); // s1 passed as a reference - not move.
                                   // but value is immutable - can't change!
    println!("The length of '{s1}' is {len}.");
}
fn calculate_length_ref(s: &String) -> usize {
    s.len()
}  // Nothing happens with s, because it was not owned.


fn mutable_reference_test() {
    println!("### mutable_reference_test:");
    let mut s = String::from("hello");

    reference_and_change(&mut s); // BUT ONLY ONE MUTABLE REF is possible.

    println!("{s}.");
}

fn reference_and_change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn two_mutable_references_test() {

    println!("### two_mutable_references_test:");
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{s}.");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{s}.");
}