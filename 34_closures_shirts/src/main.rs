/*
https://doc.rust-lang.org/stable/book/ch13-01-closures.html

Here’s the scenario: Every so often, our t-shirt company gives away an exclusive, 
limited-edition shirt to someone on our mailing list as a promotion. 
People on the mailing list can optionally add their favorite color to their profile. 
If the person chosen for a free shirt has their favorite color set, they get that color shirt. 
If the person hasn’t specified a favorite color, they get whatever color the company currently has the most of.
*/
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // CLOSURE with NO PARAMETERS
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn generate_shirts_giveaways() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None; // HERE CLOSURE WILL BE EXECUTED
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    generate_shirts_giveaways();


    /*
        Closure might:
        FnOnce      taking ownership
        FnMut       borrowing mutably
        Fn          borrowing immutably

        Once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined (thus affecting what, if anything, is moved into the closure),
        the code in the body of the closure defines what happens to the references or values when the closure is evaluated later (thus affecting what, if anything, is moved out of the closure). 
        A closure body can do any of the following: move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment to begin with.

        The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. 
        Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
        
        FnOnce       applies to closures that can be called ONCE. All closures implement AT LEAST this trait because all closures can be called. 
                    A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.


        FnMut        applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
        Fn           applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment.
                    These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times CONCURRENTLY.

    */

    // Example of  Fn - borrowing immutably
    let list = vec![1, 2, 3];
    println!("Borrowing immutably. Before defining closure: {list:?}");
    let only_borrows = || println!("Borrowing immutably. From closure: {list:?}");
    println!("Borrowing immutably. Before calling closure: {list:?}");
    only_borrows();
    println!("Borrowing immutably. After calling closure: {list:?}");

    // Example of  FnMut - borrowing mutably    
    let mut list2 = vec![1, 2, 3];
    println!("Borrowing mutably. Before defining closure: {list2:?}");
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("Borrowing mutably. After calling closure: {list2:?}");

    // Example of  FnOnce - taking ownership
    // requires use std::thread;
    let list = vec![1, 2, 3];
    println!("Taking ownership. Before defining closure: {list:?}");
    // We spawn a new thread, giving the thread a closure to run as an argument. The closure body prints out the list.
    // Even though the closure body still only needs an immutable reference, 
    // we need to specify that list should be moved into the closure by putting 
    // the MOVE keyword at the beginning of the closure definition. 

    // The new thread might finish before the rest of the main thread finishes, 
    // or the main thread might finish first. If the main thread maintained 
    // ownership of list but ended before the new thread did and dropped list,
    // the immutable reference in the thread would be invalid. Therefore, 
    // the compiler requires that list be moved into the closure given to 
    // the new thread so the reference will be valid. 
    thread::spawn(move || println!("Taking ownership. From thread: {list:?}"))
        .join()
        .unwrap();

    /*
    FnOnce example

    Let’s look at the definition of the unwrap_or_else method on Option<T> that we used in Listing 13-1:

    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }
    The trait bound specified on the generic type F is FnOnce() -> T, 
    which means F must be able to be called once, take no arguments, and return a T. 
    Using FnOnce in the trait bound expresses the constraint that unwrap_or_else 
    is only going to call f at most one time. In the body of unwrap_or_else, 
    we can see that if the Option is Some, f won’t be called. If the Option is None, 
    f will be called once. Because all closures implement FnOnce,
    unwrap_or_else accepts all three kinds of closures and is as flexible as it can be.

    Note: If what we want to do doesn’t require capturing a value from the environment, 
    we can use the name of a function rather than a closure. For example, we could call 

        unwrap_or_else(Vec::new)  

    on a Option<Vec<T>> value to get a new, empty vector if the value is None. 
    The compiler automatically implements whichever of the Fn traits is applicable for a function definition.
    */

    /*
    FnMove example

    The reason sort_by_key is defined to take an FnMut closure is that it calls 
    the closure multiple times: once for each item in the slice. 
    The closure |r| r.width doesn’t capture, mutate, or move out anything from its environment, 
    so it meets the trait bound requirements.
    */
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    
    /*
    FnOnce example

    In contrast, this example of a closure that implements just the FnOnce trait, 
    because it moves a value out of the environment. The compiler won’t let us use this closure with sort_by_key:
    */
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    /*
        let mut sort_operations = vec![];
        let value = String::from("closure called");

        list.sort_by_key(|r| {
            sort_operations.push(value);
            r.width
        });
        println!("{list:#?}");


    This is a contrived, convoluted way (that doesn’t work) to try and count the number of times sort_by_key calls the closure when sorting list. 
    This code attempts to do this counting by pushing value—a String from the closure’s environment—into the sort_operations vector.
    The closure captures value and then moves value out of the closure by transferring ownership of value to the sort_operations vector. 
    This closure can be called once; trying to call it a second time wouldn’t work because value would no longer be in the environment to be pushed into sort_operations again! 
    Therefore, this closure only implements FnOnce. When we try to compile this code, we get this error that value can’t be moved out of the closure because the closure must implement FnMut.
    The error points to the line in the closure body that moves value out of the environment. 

    To fix this, we need to change the closure body so that it doesn’t move values out of the environment. 
    Keeping a counter in the environment and incrementing its value in the closure body is a more straightforward way to count the number of times the closure is called. 
    The closure in Listing 13-9 works with sort_by_key because it is only capturing a mutable reference to the num_sort_operations counter and can therefore be called more than once:
    */

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");

    /*
    The Fn traits are important when defining or using functions or types that make use of closures. 
    In the next section, we’ll discuss iterators. 
    Many iterator methods take closure arguments, so keep these closure details in mind as we continue!
    */
}