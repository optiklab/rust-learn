#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Another good similar example is with snappy data compression lib in https://doc.rust-lang.org/nomicon/ffi.html