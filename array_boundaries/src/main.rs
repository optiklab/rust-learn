use std::io;

fn print_coordinates(x: i32, y: u32, z: f64) {
    
    println!("x: {0}", x);
    println!("y: {0}", y);
    println!("z: {0}", z);
}

fn print_array_element(i: i32) {
    
    println!("Element: {}", i);
}

fn main() {
    let a = [ 0, 1, 2, 3, 4, 5 ];
    print_array_element(a[0]);

    let b: [i32; 5] = [ 0, 1, 2, 3, 4 ];
    print_array_element(b[0]);

    let c = [77; 10]; // Array of 10 i32 elements, each of value 5
    print_array_element(c[0]);

    let tup: (i32, u32, f64) = (-500, 500, 5.6);
    let (x, y, z) = tup;
    print_coordinates(x, y, z);

    let my_x = tup.0;
    let my_y = tup.1;
    let my_z = tup.2;
    print_coordinates(my_x, my_y, my_z);

    loop {
        println!("Type an index:");

        let mut index = String::new();

        std::io::stdin().read_line(&mut index)
            .expect("Failed to read line"); 

        let index_number : u32 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Index selected: {}", index_number);
        break;
    }
}
