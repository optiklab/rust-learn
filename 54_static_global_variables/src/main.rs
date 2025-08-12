//  The names of static variables are in SCREAMING_SNAKE_CASE by convention:
static HELLO_WORLD: &str = "Hello, world!";

// Static variables can only store references with the 'static lifetime, 
// which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly. 

// Accessing an immutable static variable is safe.
fn main() {
    println!("name is: {HELLO_WORLD}");

    modify_static_var();
}

// A subtle difference between constants and immutable static variables is that 
// values in a static variable have a fixed address in memory.
// Using the value will always access the same data. 
// Constants, on the other hand, are allowed to duplicate their data whenever they’re used.

// Another difference is that static variables can be mutable. Accessing 
// and modifying mutable static variables is unsafe. 
static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    // With mutable data 
    // that is globally accessible, it’s difficult to ensure there are no data races, 
    // which is why Rust considers mutable static variables to be unsafe. 
    unsafe {
        COUNTER += inc;
    }
}

fn modify_static_var() {
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
        // Compiler will not allow you to create references to a mutable static variable. 
        // You can only access it via a raw pointer *, created with one of the &raw borrow operators. 
    }
}

// Whenever we write an unsafe function, it is idiomatic to write a comment 
// starting with SAFETY and explaining what the caller needs to do to call 
// the function safely. Likewise, whenever we perform an unsafe operation,
// it is idiomatic to write a comment starting with SAFETY to explain how 
// the safety rules are upheld.