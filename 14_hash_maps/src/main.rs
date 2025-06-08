use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); // HashMap with ttps://en.wikipedia.org/wiki/SipHash hasher

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name) // get method returns an Option<&V>
        // None if the key is not found
        // Option<&i32> if key is found
        .copied() // Convert Option<&i32> to Option<i32>
        .unwrap_or(0); // Unwrap the Option, or return 0 if not found.

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    // Update
    scores.insert(String::from("Blue"), 25);
    println!("Updated: {scores:?}");

    // Add only if key does not exist
    scores.entry(String::from("Blue")).or_insert(500);    
    scores.entry(String::from("Green")).or_insert(500);

    println!("Updated: {scores:?}");

    // Ownership and borrowing in HashMaps
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Counting words in a string
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // Dereference count to get the value and increment it
    }
    println!("{map:?}");

}
