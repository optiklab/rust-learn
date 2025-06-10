use integration_tests::add_two; // Each file in the tests directory is a separate crate, 
                                // so we need to bring our library into each test crate’s scope.

#[test] // We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
        // Cargo treats the tests directory specially and compiles files in this directory only when we run 'cargo test'.
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}