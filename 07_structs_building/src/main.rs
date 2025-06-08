struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    tuples_usage();

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user1.email: {}", user1.email);

    let mut user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // We can no longer use user1 after creating user2
    // because the String in the username field of user1 was moved into user2

    println!("user2.email: {}", user2.email);
    user2.email = String::from("anotheremail@example.com");
    println!("user2.email: {}", user2.email);

    let user3 = User {
        email: String::from("puff@example.com"),
        // ..user1 // COMPILE ERROR : already moved
        ..user2
    };
    println!("user3.email: {}", user3.email);

    let user4 = build_user("email@email.com".to_string(), "Email Emailovich".to_string());
    println!("user4.email: {}", user4.email);

    tuple_structs_wo_named_fileds();

    let subject = AlwaysEqual;

    create_rect();
}

fn tuples_usage() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
/*      username: username,
        email: email,*/
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple Structs when naming fields would be verbose or redundant.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs_wo_named_fileds() {
    let black = Color(0, 0, 0);
    println!("black: r{} g{} b{}", black.0, black.1, black.2);

    let origin = Point(0, 0, 0);
    println!("black: x{} y{} z{}", origin.0, origin.1, origin.2);
}

// Unit-Like Struct Without Any Fields when you need to implement a trait 
// on some type but don’t have any data to store in the type itself. 
struct AlwaysEqual;

/*
// COMPILE ERROR: Lifetime specifiers required
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
*/

//#############################
#[derive(Debug)] // To be able to print with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

fn create_rect() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.", 
        area_rect(&rect1)
    );
    println!("Rect is {:?}", rect1);
    println!("Rect is {:#?}", rect1);
}

fn area_rect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}