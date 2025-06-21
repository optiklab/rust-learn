https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

Cargo.toml is about describing your dependencies in a broad sense, and is written by you.
Cargo.lock contains exact information about your dependencies. It is maintained by Cargo and should not be manually edited.

    ```
    godbolt.org > rustc > --emit=llvm-ir
                            -C opt-level=3
                            -C opt-level=2
                            -C opt-level=1
    ```

    $> rustup

    $> cd hello_world
    $> rustc main.rs

    $> cargo --version
    
    $> cargo update


#Create Binary (aka App) crate/package

    $> cargo new hello_cargo

    $> cd hello_cargo

    'hello_cargo' is the crate or package. And root module in the same time by default.
    However, you can define many modules and make some (or all) of them public.

    Binary crates are executable, while Lib crates are used as depenencies.
    Binary crates published to crates.io are installable:
    $> cargo install ripgrep

#Create Lib package

    $> cargo new adder --lib

#Alternatively, create Workspace for more complex project of many packages

    Create project folder and create Cargo.toml file manually.

    Put in the Cargo.toml workspace tag:
    ```
    [workspace]
    resolver = "3"
    ```

    Resolver = 3 is a version of resolver algorithm.

    Then create project within that workspace:
    $> cargo new adder

    You will notice new "members" field if Cargo.toml with project added.
    Creating lib project within this folder will add one more LIB project into members list:
    $> cargo new add_one --lib

    'target' folder will be shared between those projects.

    Add lib project as a dependency to the App project:
    ```
    [dependencies]
    add_one = { path = "../add_one" }
    ```

    Dependencies ALWAYS added manually for each separate crate. Even if there duplicated
    public crates that we depend on. During build Cargo will make sure that every
    package uses the same version of public crate and will take care of 
    efficient usage by using single copy of public crate for all packages in workspace.

#Build package

    $> cargo check - just to check if it compiles

    Docs https://doc.rust-lang.org/stable/book/ch14-01-release-profiles.html:
    $> cargo build
    $> cargo build
        Builds `dev` profile [unoptimized + debuginfo] target(s) configured as [profile.dev] in Cargo.toml.
    $> cargo build --release
        Builds `release` profile [optimized] target(s) configured as [profile.release] in Cargo.toml.

#Run package

    $> cargo run
    $> RUST_BACKTRACE=1 cargo run

#Run package from Workspace

    Workspace usually contains multiple packages, so we need to specify which to run:
    $> cargo run -p adder

#Open docs and run doc tests

    $> cargo doc --open

#Explore

    $> cargo install cargo-expand
    $> cargo expand - to see how "derived" actually look like

#Tests

##Unit tests

    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
        └── main.rs  <-- both code and unit-tests are here
        or        
        └── lib.rs  <-- both code and unit-tests are here

##Integration tests

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

    Run test for a package/module. It runs both unit and integrations tests,
    as well as also documentation tests.
    $> cargo test

    Run tests for a specific package within Workspace:
    $> cargo test -p add_one

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

#Publish crates

    Follow https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html
    to re-export your modules in a necessary and user-friendly way.
    
    Then make sure all the descriptions fields are filled in Cargo.toml:
    [package]
    name = "guessing_game"
    version = "0.1.0"
    edition = "2024"
    description = "A fun game where you guess what number the computer has chosen."
    license = "MIT OR Apache-2.0"

    Follow https://doc.rust-lang.org/cargo/ to fill all the necessary [metadata](https://doc.rust-lang.org/cargo/reference/manifest.html).
    
    Then publish:
    $> cargo publish

    We also can publish specific packages/crates in our Workspace:
    $> cargo publish -p add_one


    Or deprecating versions if you need to:
    $> cargo yank --vers 1.0.1

    Or stop deprecating version:
    $> cargo yank --vers 1.0.1 --undo