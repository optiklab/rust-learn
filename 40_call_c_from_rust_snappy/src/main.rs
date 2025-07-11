use libc::size_t;

// vcpkg install snappy
// TODO 
//    copy from VCPKG_ROOT/installed/{PLATFORM}/bin/snappy.lib
//    copy from VCPKG_ROOT/installed/{PLATFORM}/bin/snappy.dll
// to
//    /40_call_c_from_rust_snappy/ top level project folder

#[link(name = "snappy")] // Declare a name of a linking lib: https://github.com/google/snappy/
unsafe extern "C" {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);
}