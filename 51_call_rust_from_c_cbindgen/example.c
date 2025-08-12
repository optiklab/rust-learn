#include <stdio.h>
#include "call_rust_from_c_cbindgen.h"

// Declare the Rust function
void call_from_c();

int main() {
    printf("Calling Rust from C...\n");
    call_from_c();
    return 0;
}

// Another good similar example is with snappy data compression lib in https://doc.rust-lang.org/nomicon/ffi.html