use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let rand_secret : u32 = rand::rng().random_range(1..11);


    loop {
        println!("Guess a number:");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess) // There is also io::Result returned. Enum with Ok and Err values.
            .expect("Failed to read line"); // An error of not handled return if you don't call expect().

        let guess_number: u32 = guess.trim().parse()
            .expect("Please, type non negative number!");

        println!("A number guessed is {}", guess_number);

        match guess_number.cmp(&rand_secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YESSS! Bye!");
                return;
            }
        }

        println!("Secret number was: {}! Now you know!", rand_secret);
    }
}
