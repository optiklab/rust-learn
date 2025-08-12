

// cbindgen has a simple but effective strategy. It walks through your crate looking for:
//    #[no_mangle] pub extern fn ("functions")
//    #[no_mangle] pub static ("globals")
//    pub const ("constants")
// and generates a header declaring those items. 

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// https://github.com/mozilla/cbindgen/

// https://github.com/mozilla/cbindgen/blob/master/docs.md
