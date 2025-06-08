struct MyObject {
    name: String,
    value: i32,
}

fn main() {
    let mut my_vec: Vec<MyObject> = Vec::new();

    my_vec.push(MyObject {
        name: String::from("Object1"),
        value: 10,
    });
    my_vec.push(MyObject {
        name: String::from("Object2"),
        value: 20,
    });

    for (i, obj) in my_vec.iter().enumerate() {
        println!("Object at index {}: name = {}, value = {}", i, obj.name, obj.value);
    }

    let mut sec_obj = my_vec[1].name.clone(); // Clone the name to make it mutable
    sec_obj.push_str(" - Updated");
    my_vec[1].name = sec_obj; // Update the name of the second object
    println!("Updated second object: name = {}, value = {}", my_vec[1].name, my_vec[1].value);

    for (i, obj) in my_vec.iter().enumerate() {
        println!("Object at index {}: name = {}, value = {}", i, obj.name, obj.value);
    }

    let mut second_object: Option<&MyObject> = my_vec.get(1);
    match second_object {
        Some(obj) => { // `obj` is a `&` reference, so the data it refers to cannot be written
            //obj.name = "UpdatedObject2".to_string(); // This will not compile, as `name` is not mutable
            println!("Second object: name = {}, value = {}", obj.name, obj.value);
        },
        None => println!("No second object found"),
    }
   // second_object.name = "UpdatedObject2".to_string(); // This will not compile, as `name` is not mutable
}