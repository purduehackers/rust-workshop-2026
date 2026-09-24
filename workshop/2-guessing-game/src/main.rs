//! A guessing game program written in Rust
//! to get a taste of the language. This code
//! was taken from The Rust Programming Lanugage
//! book.
//!
//! Run this program by entering the following
//! command into your terminal from the
//! `workshop` directory:
//!
//! cargo run --bin variables

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a randomly chosen secret number
    // between 1 and 100 inclusively
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Read user input from the command line
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the user input into a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // If not a number, retry
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare the guess to the randomly chosen
        // secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
