fn print_coordinates(x: i32, y: u32, z: f64) {
    
    println!("x: {0}", x);
    println!("y: {0}", y);
    println!("z: {0}", z);
}

fn print_array_element(i: i32, index: usize) {
    
    println!("Element {}: {}", index, i);
}

fn main() {

    let (x, y, z) = (1, 2, 3); // Not an array. Destructure pattern!

    print_from_loop();
    print_from_for();
    print_from_for_rev();

    println!("###let a = [ 0, 1, 2, 3, 4, 5 ];");
    let a = [ 0, 1, 2, 3, 4, 5 ];
    print_array_element(a[0], 0);
    let mut counter = 0;
    for el in a.iter() {
        print_array_element(*el, counter);
        counter += 1;
    }
    println!("END");

    println!("###let b: [i32; 5] = [ 0, 1, 2, 3, 4 ];");
    let b: [i32; 5] = [ 0, 1, 2, 3, 4 ];
    print_array_element(b[0], 0);
    let bslice = &b[1..3];
    for (i, el) in bslice.iter().enumerate() {
        print_array_element(*el, i);
    }
    println!("END");

    println!("###let c = [77; 10]");
    let c = [77; 10]; // Array of 10 i32 elements, each of value 5
    print_array_element(c[0], 0);    
    println!("END");

    println!("###let tup: (i32, u32, f64) = (-500, 500, 5.6);");
    let tup: (i32, u32, f64) = (-500, 500, 5.6);
    let (x, y, z) = tup;
    print_coordinates(x, y, z);
    println!("END");

    println!("### my_x = tup.0:");
    let my_x = tup.0;
    let my_y = tup.1;
    let my_z = tup.2;
    print_coordinates(my_x, my_y, my_z);
    println!("END");

    println!("###!Warning! Potentially unsafe behavior! Get an element from array by index!");

    loop {
        println!("Type an index:");

        let mut index = String::new();

        std::io::stdin().read_line(&mut index)
            .expect("Failed to read line"); 

        let index_number : usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Index selected: {}", index_number);
        println!("Element Found by Index: {}", c[index_number]); // POTENTIAL PANIC!!!
        break;
    }
    println!("END");

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    println!("### Months print:");
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    
    for &month_ref in months.iter() {
        println!("{month_ref}");
    }    
    for month in months.iter() {
        println!("{month}");
    }

    //Alternative:
    // for (i, &month) in months.iter().enumerate() {
    // ...
    // }
}

fn print_from_loop() {
    
    println!("###let result = loop {{ counter += 1; ...");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Value: {}", result);
    println!("END");
}

fn print_from_for() {

    println!("###for number in 1..4:");
    for number in 1..4  {
        println!("{}", number);
    }
    println!("END");
}

fn print_from_for_rev() {

    println!("###for number in (1..4).rev():");
    for number in (1..4).rev()  {
        println!("{}", number);
    }
    println!("END");
}