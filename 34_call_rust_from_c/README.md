# GCC (BASE)

On Windows, Rust by default builds a .rlib (Rust static library), 
not a C-compatible .dll or .lib. To call Rust from C, you need a 
dynamic library (cdylib). Here’s how to fix your setup:

1. Update Cargo.toml to build a C-compatible dynamic library
Add or modify the [lib] section:
    crate-type = ["cdylib"]

to build DLL instead of LIB

2. Build Rust library:

    cargo clean
    cargo build --release

3. Link with GCC (make sure to put EXE file in the same folder with DLL)
    gcc example.c -Ltarget/release -lcall_rust_from_c -o target/release/example.exe


4. Run:
    .\target\release\example.exe
        Calling Rust from C...
        Just called a Rust function from C!

    YAY!

# CLANG

To link with CLANG it's not enough to only compile:
    clang example.c -Ltarget/release -llibcall_rust_from_c -o example.exe

 Clang (actually, the MSVC linker) is looking for an import library (call_rust_from_c.lib) to link against your DLL, but Rust only generates the .dll by default. To generate the .lib file, you need to install the [MSVC toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html#toolchains) and build with it.

 a. Check your toolchain:
    rustup show

    If you see x86_64-pc-windows-msvc as your default, you're good.

b. If not, install and set it:
    rustup toolchain install stable-x86_64-pc-windows-msvc
    rustup default stable-x86_64-pc-windows-msvc

c. Rebuild the lib and link with clang:
    cargo clean
    cargo build --release
    clang example.c -Ltarget/release -lcall_rust_from_c -o example.exe