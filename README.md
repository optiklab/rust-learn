https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

godbolt.org > rustc > --emit=llvm-ir
                        -C opt-level=3
                        -C opt-level=2
                        -C opt-level=1

    $> rustup

    $> cd hello_world
    $> rustc main.rs

    $> cargo --version
    
    $> cargo update

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

    #Unit tests

    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
        └── main.rs  <-- both code and unit-tests are here
        or        
        └── lib.rs  <-- both code and unit-tests are here

    #Integration tests

    We cannot create integration tests for App project. Only for lib-project:
    $>cargo new project_with_integration_tests --lib

    This is because each integration tests file is a care. And only
    library crates expose functions that other crates can use; binary crates 
    are meant to be run on their own.

    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── lib.rs  <-- both code and unit-tests are here
    └── tests       <-- integration tests under 'tests' folder
        ├── common  <-- Files in subdirectories of the tests directory 
                        don’t get compiled as separate crates and 
                        don't have sections in the test output.
        │   └── mod.rs <-- shared code for tests
        └── integration_test.rs  <-- each file - separate crate
        └── common.rs  <-- each file - separate crate. Wrong way to share code.
                           Will also appear in the test output!

#Build & run tests

    $> cargo test

    $> cargo test -- --test-threads=1
                ^ SEPARATOR --

    Some command line options go to cargo test, and some go to the resultant test binary. To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary. Running cargo test --help displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator. Those options are also documented in the [Tests section](https://doc.rust-lang.org/rustc/tests/index.html) of the the [rustc book](https://doc.rust-lang.org/rustc/index.html).

    To run single test (either unit or integration test):
    $> cargo test one_hundred

    To run all tests that start from 'add' (either unit or integration tests):
    $> cargo test add

    To run integration tests crate (in file integration_test_file_name.rs):
    $> cargo test --test integration_test_file_name

    To run ignored:
    $> cargo test -- --ignored

    By default, if a test passes, Rust’s test library captures anything printed to 
    standard output. So you will not see any details in terminal. If we want to see 
    printed values for passing tests as well, we can tell Rust to also show 
    the output of successful tests with --show-output or --nocapture:

    $> cargo test -- --show-output
    OR
    $> cargo test -- --nocapture

    If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads:
    $> cargo test -- --test-threads=1