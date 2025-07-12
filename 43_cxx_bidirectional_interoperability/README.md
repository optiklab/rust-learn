Following tutorial: https://cxx.rs/tutorial.html

Source code: https://github.com/dtolnay/cxx

CXX comprises two code generators: a Rust one (which is the cxx::bridge attribute procedural macro) and a C++ one.

# Rust generated code

It's easiest to view the output of the procedural macro by installing cargo-expand. Then run cargo expand ::ffi to macro-expand the mod ffi module.

cxx-demo$  cargo install cargo-expand
cxx-demo$  cargo expand ::ffi

You'll see some deeply unpleasant code involving #[repr(C)], #[link_name], and #[export_name].

# C++ generated code

For debugging convenience, cxx_build links all generated C++ code into Cargo's target directory under target/cxxbridge/.

cxx-demo$  exa -T target/cxxbridge/
target/cxxbridge
├── cxx-demo
│  └── src
│     ├── main.rs.cc -> ../../../debug/build/cxx-demo-11c6f678ce5c3437/out/cxxbridge/sources/cxx-demo/src/main.rs.cc
│     └── main.rs.h -> ../../../debug/build/cxx-demo-11c6f678ce5c3437/out/cxxbridge/include/cxx-demo/src/main.rs.h
└── rust
   └── cxx.h -> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cxx-1.0.0/include/cxx.h

In those files you'll see declarations or templates of any CXX Rust types present in your language boundary (like rust::Slice<T> for &[T]) and extern "C" signatures corresponding to your extern functions.

If it fits your workflow better, the CXX C++ code generator is also available as a standalone executable which outputs generated code to stdout.

cxx-demo$  cargo install cxxbridge-cmd
cxx-demo$  cxxbridge src/main.rs
