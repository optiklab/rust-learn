https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

godbolt.org > rustc > --emit=llvm-ir
                        -C opt-level=3
                        -C opt-level=2
                        -C opt-level=1

$> rustup

$> cd hello_world
$> rustc main.rs


$> cargo --version

#Create App project

$> cargo new hello_cargo

$> cd hello_cargo

#Create lib project

$>cargo new adder --lib

#Build and run project

$> cargo build

$> cargo run
$> RUST_BACKTRACE=1 cargo run

$> cargo check - just to check if it compiles


$> cargo new guessing_game
(add rand library crate as a dependency)

$> cargo update

$> cargo doc --open

$> cargo new array_boundaries


#Tests

$> cargo test

$> cargo test -- --test-threads=1
               ^ SEPARATOR --

Some command line options go to cargo test, and some go to the resultant test binary. To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary. Running cargo test --help displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator. Those options are also documented in the [Tests section](https://doc.rust-lang.org/rustc/tests/index.html) of the the [rustc book](https://doc.rust-lang.org/rustc/index.html).

If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads:
$> cargo test -- --test-threads=1

By default, if a test passes, Rust’s test library captures anything printed to 
standard output. So you will not see any details in terminal. If we want to see 
printed values for passing tests as well, we can tell Rust to also show 
the output of successful tests with --show-output or --nocapture:

$> cargo test -- --show-output
OR
$> cargo test -- --nocapture

To run single test:
$>cargo test one_hundred

To run all tests that start from add:
$>cargo test add

To run ignored:
cargo test -- --ignored