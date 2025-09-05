use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // You can use the `use` keyword to bring module paths from modules from
    // anywhere and especially from the standard library into your scope.
    // For example, we bring `SystemTime` and `UNIX_EPOCH`
    // from the `std::time` module into your scope.
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    // By default module parts are private.
    sausage_factory::make_sausage(); // PUB!

    // You can bring module paths into scopes and provide new names for them with
    // the `use` and `as` keywords.
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {  // PUB!
        get_secret_recipe();
        println!("sausage!");
    }
}

// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.
mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;  // PUB!
    pub use self::veggies::CUCUMBER as veggie;  // PUB!

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}
