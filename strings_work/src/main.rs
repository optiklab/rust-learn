fn main() {
    let mut s: char = 'a'; // RUSTC:    mov     dword ptr [rsp - 4], 97

    let mut s1 = String::new();
    s1 = "Anton".to_string();

    s1.push_str(" Yarkov"); // OR +=

    println!("s1: {}", s1);

    let mut s2 = String::from("Hello");
    s2 += " World!"; // OR s2.push_str(...)
    println!("s2: {}", s2);

    let s3 = s2; // MOVE
    println!("s3: {} (s2 moved to s3)", s3);
    println!("(s2 freed)");
    //println!("{}", s2); // COMPILE ERROR

    let s4 = s3.clone(); // CLONE
    println!("s4: {} (s3 cloned to s4)", s4);
    println!("s3: {} (s3 still alive)", s3);

    copy_or_not();
}

pub fn copy_or_not() {
    let tup1: (i32, i32) = (1, 2);
    let (x, y) = tup1; // COPY

    let tup2: (i32, String) = (1, "2".to_string());
    let (x, y) = tup2; // MOVE
}