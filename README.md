https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

godbolt.org > rustc > --emit=llvm-ir
                        -C opt-level=3
                        -C opt-level=2
                        -C opt-level=1


rustup

cd hello_world
rustc main.rs

cargo --version
cargo new hello_cargo
cd hello_cargo
cargo build
cargo run
RUST_BACKTRACE=1 cargo run
cargo check - just to check if it compiles

cargo new guessing_game
(add rand library crate as a dependency)
cargo update

cargo doc --open

cargo new array_boundaries


