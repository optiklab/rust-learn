
/*
// Normal usual way.
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
*/

// However, in case of just ABC standard function from Lib C,
// we know it is very safe. So we may simplify the call:

unsafe extern "C" {
    safe fn abs(input: i32) -> i32; // This is a Standard Lib function. No #[link] declarations required.
}
// You're telling the Rust compiler:
// “There exists a function named abs with C calling conventions, 
// and it takes an i32 and returns an i32.”
// But this declaration does not link the function automatically. 
// Here's how Rust makes it work:
// 1. The extern "C" block tells Rust the function exists and uses the C ABI.cd ..
//    Rust trusts that you’ve declared the function correctly.
// 2. No Implementation: Rust doesn't provide the implementation—it expects the linker to find it. 
//    If the function doesn’t exist or is declared incorrectly due to wrong signature,
//    you’ll get a link-time error or undefined behavior at runtime.
// 3. On most platforms, the C standard library (like libc) is linked by default. 
//    So when you compile your Rust program, the linker finds abs in libc.
//    On Unix-like systems, libc is always available, so functions like abs, 
//    printf, etc., are usually safe to call.

fn main() {
    println!("Absolute value of -3 according to C: {}", abs(-3));
}

// Safer alternative: instead of declaring abs manually, you can use the libc crate:
//# Cargo.toml
//[dependencies]
//libc = "0.2"

// Another good similar example is with snappy data compression lib in https://doc.rust-lang.org/nomicon/ffi.html
