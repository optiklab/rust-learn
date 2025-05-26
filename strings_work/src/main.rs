fn main() {
    let mut s: char = 'a'; // RUSTC:    mov     dword ptr [rsp - 4], 97

    let mut s1 = String::new();
    s1 = "Anton".to_string();

    s1.push_str(" Yarkov"); // OR +=

    println!("{}", s1);

    let mut s2 = String::from("Hello");
    s2 += " World!"; // OR s2.push_str(...)
    println!("{}", s2);
}
