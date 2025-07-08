#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let (x, y, z) = (1, 2, 3); // Not an array. Destructure pattern!
    
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    
    // Prints: State quarter from Alabama!
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    println!("{:?}", five); // Prints: Some(5)

    let six = plus_one(five);
    println!("{:?}", six); // Prints: Some(6)

    let none = plus_one(None);
    println!("{:?}", none); // Prints: None

    // 'other' is a catch-all and bind value pattern
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    }

    // _ is a catch-all without binding pattern
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => println!("_ ignored rest of values")
    }

    // if let is a convenient way to match a single pattern
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // Enums with methods to check value:
    let state: UsState = UsState::Alaska;
    if state.existed_in(1959) {
        println!("Alaska became a state in 1959!");
    } else {
        println!("Alaska was not a state in 1959.");
    }
    // Or:
    describe_state_quarter(Coin::Quarter(UsState::Alaska));
    describe_state_quarter_shorter(Coin::Quarter(UsState::Alaska));
    describe_state_quarter_refactored(Coin::Quarter(UsState::Alaska));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing fancy hat!");
}

fn move_player(num_spaces: u8)  {
    println!("Moving player {num_spaces} spaces!");
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_refactored(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_shorter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}