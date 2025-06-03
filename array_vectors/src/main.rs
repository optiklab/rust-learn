use std::fmt;

// When you need to store different types of data in a single vector, you can use enums.
// This is useful for representing different types of cells in a spreadsheet.
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// Implementing the Display trait for SpreadsheetCell allows us to print the enum variants in a user-friendly way.
impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpreadsheetCell::Int(i) => i.to_string().fmt(f),
            SpreadsheetCell::Float(fl) => fl.to_string().fmt(f),
            SpreadsheetCell::Text(t) => t.to_string().fmt(f)
        }
    }
}

fn main() {
    let mut my_vec: Vec<i32> = Vec::new();
    println!("my_vec # Initial vector length: {}", my_vec.len());
    println!("my_vec # Initial vector capacity: {}", my_vec.capacity());
    my_vec.push(1);
    println!("my_vec # After pushing 1, vector length: {}", my_vec.len());
    println!("my_vec # After pushing 1, vector capacity: {}", my_vec.capacity());
    my_vec.push(2);
    println!("my_vec # After pushing 2, vector length: {}", my_vec.len());
    println!("my_vec # After pushing 2, vector capacity: {}", my_vec.capacity());
    my_vec.extend([3, 4, 5]);
    println!("my_vec # After extending with [3, 4, 5], vector length: {}", my_vec.len());
    println!("my_vec # After extending with [3, 4, 5], vector capacity: {}", my_vec.capacity());
    for (i, value) in my_vec.iter().enumerate() {
        println!("my_vec # Element at index {}: {}", i, value);
    }

    let third: &i32 = &my_vec[2];
    println!("my_vec # Third element: {}", third);
    let second: Option<&i32> = my_vec.get(1);
    match second {
        Some(value) => println!("my_vec # Second element: {}", value),
        None => println!("my_vec # No second element found"), // No panic here
    }
    match my_vec.get(0) {
        Some(value) => println!("my_vec # First element: {}", value),
        None => println!("my_vec # No first element found") // No panic here
    }
    
    let does_not_exist: Option<&i32> = my_vec.get(10); // No panic here, None
    //let does_not_exist2 = my_vec[10]; // This will panic if uncommented
    
    println!("############################################");

    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    for i in 0..vec.len() {
        println!("vec #Element at index {}: {}", i, vec[i]);
    }
    println!("vec #Vector length: {}", vec.len());
    println!("vec #Vector capacity: {}", vec.capacity());
    vec.push(6);
    println!("vec #After pushing 6, vector length: {}", vec.len());
    println!("vec #After pushing 6, vector capacity: {}", vec.capacity());
    vec.pop();
    println!("vec #After popping, vector length: {}", vec.len());
    println!("vec #After popping, vector capacity: {}", vec.capacity());
    vec.clear();
    println!("vec #After clearing, vector length: {}", vec.len());
    println!("vec #After clearing, vector capacity: {}", vec.capacity());
    vec.extend([7, 8, 9]);
    println!("vec #After extending with [7, 8, 9], vector length: {}", vec.len());
    println!("vec #After extending with [7, 8, 9], vector capacity: {}", vec.capacity());
    for (i, value) in vec.iter().enumerate() {
        println!("vec #Element at index {}: {}", i, value);
    }
    vec.sort();
    println!("vec #After sorting, vector elements:");
    for value in &vec {
        println!("{}", value);
    }
    vec.reverse();
    println!("vec #After reversing, vector elements:");
    for value in &vec {
        println!("{}", value);
    }
    let mut vec2 = vec![10, 20, 30];
    vec.append(&mut vec2);
    println!("vec #After appending vec2, vector elements:");
    for value in &vec {
        println!("{}", value);
    }
    println!("vec #vec2 after appending: {:?}", vec2);

    vec.retain(|&x| x % 2 == 0);
    println!("vec #After retaining even elements, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.dedup();
    println!("vec #After deduplicating, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }

    vec.truncate(2);
    println!("After truncating to 2 elements, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.shrink_to_fit();
    println!("vec #After shrinking to fit, vector capacity: {}", vec.capacity());
    vec.reserve(10);

    println!("vec #After reserving space for 10 more elements, vector capacity: {}", vec.capacity());
    vec.shrink_to(5);
    println!("vec #After shrinking to 5 elements, vector length: {}", vec.len());
    println!("vec #After shrinking to 5 elements, vector capacity: {}", vec.capacity());
    vec.resize(8, 0);
    println!("vec #After resizing to 8 elements, vector elements:");
    for value in &vec {
        println!("{}", value);
    }
    vec.resize(3, 0);
    println!("vec #After resizing to 3 elements, vector elements:");

    for value in &vec {
        println!("vec #{}", value);
    }
    vec.sort_unstable();
    println!("vec #After unstable sorting, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.reverse();
    println!("vec #After reversing again, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.clear();
    println!("vec #After clearing again, vector length: {}", vec.len());
    println!("vec #After clearing again, vector capacity: {}", vec.capacity());
    vec.extend(vec![11, 12, 13]);
    println!("vec #After extending with [11, 12, 13], vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.sort_by(|a, b| b.cmp(a));
    println!("vec #After sorting in descending order, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.reverse();
    println!("vec #After reversing again, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.clear();
    println!("vec #After clearing again, vector length: {}", vec.len());
    println!("vec #After clearing again, vector capacity: {}", vec.capacity());
    vec.extend(vec![14, 15, 16]);
    println!("vec #After extending with [14, 15, 16], vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.sort_unstable_by(|a, b| a.cmp(b));
    println!("vec #After unstable sorting again, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.reverse();
    println!("vec #After reversing again, vector elements:");   
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.clear();
    println!("vec #After clearing again, vector length: {}", vec.len());
    println!("vec #After clearing again, vector capacity: {}", vec.capacity());
    vec.extend(vec![17, 18, 19]);
    println!("vec #After extending with [17, 18, 19], vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.sort_by_key(|&x| x % 3);
    println!("vec #After sorting by key (x % 3), vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.reverse();
    println!("vec #After reversing again, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.clear();
    println!("vec #After clearing again, vector length: {}", vec.len());
    println!("vec #After clearing again, vector capacity: {}", vec.capacity()); 
    vec.extend(vec![20, 21, 22]);
    println!("vec #After extending with [20, 21, 22], vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    println!("vec #After sorting with partial comparison, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.reverse();
    println!("vec #After reversing again, vector elements:");
    for value in &vec {
        println!("vec #{}", value);
    }
    vec.clear();
    println!("vec #After clearing again, vector length: {}", vec.len());
    println!("vec #After clearing again, vector capacity: {}", vec.capacity());

    println!("############################################");

    // Cannot have immutable reference to a vector while it is being modified:
    //let first = &vec[0];
    //vec.push(6);
    //println!("The first element is: {first}");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for (i, value) in v.iter().enumerate() {
        println!("v # Element at index {}: {}", i, value);
    }


    
    println!("############################################");

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for (i, value) in row.iter().enumerate() {
        println!("Excel row # Element at index {}: {}", i, value);
    }
}

