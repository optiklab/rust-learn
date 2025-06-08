fn main() {
    println!("Hello, scopes!");

    let x = 5;

    // XXXX let y = (let z = 6); // Statements do not return value!!! ERROR!

    println!("outer x: {}", x);

    let y = { // An expression!
        let x = 3;
        println!("inner x: {}", x);
        x + 1  // Return value from expression!
    };
    
    println!("outer y: {}", y);
    println!("outer x: {}", x);

    let sq = square(x);

    let x: u32 = x.try_into().unwrap(); // Required to compare with square

    if sq > x {
        println!("square x: {}", sq);
    } else {
        // Panic
        assert!(time_elapsed > 0, "Something went wrong with square");
    }

    let z = if true {
        1
    } else {
        2
    };

    println!("z: {}", z);
}

fn square(x: i32) -> u32 {
    (x * x).try_into().unwrap()
}