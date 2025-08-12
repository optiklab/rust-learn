# SETUP CBINDGEN

cargo install --force cbindgen

And add an empty file:

$> cbindgen.toml

# SETUP PROJECT

On Windows, Rust by default builds a .rlib (Rust static library), 
not a C-compatible .dll or .lib. To call Rust from C, you need a 
dynamic library (cdylib). Here’s how to fix your setup:

Update Cargo.toml to build a C-compatible dynamic library
Add or modify the [lib] section:
    crate-type = ["cdylib"]

to build DLL instead of LIB

Then Build Rust library

# GENERATE BINDINGS

$> cbindgen --config cbindgen.toml --crate call_rust_from_c_cbindgen --output call_rust_from_c_cbindgen.h

# COMPILE

$> cargo clean
$> cargo build --release

Link with GCC (make sure to put EXE file in the same folder with DLL)
$> gcc example.c -Ltarget/release -lcall_rust_from_c -o target/release/example.exe
OR
$> clang example.c -Ltarget/release -lcall_rust_from_c -o target/release/example.exe

# RUN

$> .\target\release\example.exe

Calling Rust from C...
Just called a Rust function from C!

YAY!

https://github.com/mozilla/cbindgen/

https://github.com/mozilla/cbindgen/blob/master/docs.md
