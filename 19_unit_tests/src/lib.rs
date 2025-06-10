pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

// Internal functions are still accessible in tests.
fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

// You should put unit tests in the src directory in each file with the code 
// that they’re testing. The convention is to create a module named tests
// in each file to contain the test functions and
// to annotate the module with cfg(test).

// Test code is compiled only in case we run `cargo test` configuration.

#[cfg(test)] // the attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option 'test'.
mod add_tests {
    use super::*;

    // Simplest test.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 5);
    }
    
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }

    // Failing test. So we ignore it.
    #[test]    
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    // Writing tests so they return a Result<T, E> enables you 
    // to use the question mark operator in the body of tests, 
    // which can be a convenient way to write tests that should fail 
    // if any operation within them returns an Err variant.
    //
    // You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
    #[test]
    fn it_works_generic() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// Tests with String output.
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod greeting_tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
        // Assert with more info:        
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
}

// Panic tests.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

// Rectangle tests.
#[cfg(test)]
mod guess_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn less_then_one() {
        Guess::new(0);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}