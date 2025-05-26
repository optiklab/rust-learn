fn main() {
    return_cortage_by_moves();
    return_cortage_by_ref();
    mutable_reference_test();
    two_mutable_references_test();
}


fn return_cortage_by_moves() {
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
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1); // s1 passed as a reference - not move.
                                   // but value is immutable - can't change!
    println!("The length of '{s1}' is {len}.");
}
fn calculate_length_ref(s: &String) -> usize {
    s.len()
}  // Nothing happens with s, because it was not owned.


fn mutable_reference_test() {
    let mut s = String::from("hello");

    reference_and_change(&mut s); // BUT ONLY ONE MUTABLE REF is possible.

    println!("{s}.");
}
fn reference_and_change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn two_mutable_references_test() {

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{s}.");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{s}.");
}