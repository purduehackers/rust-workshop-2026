//! A program that demonstrates control flow.
//! using if statements and while loops, featuring
//! a coding challenge.
//!
//! Run this program by entering the following
//! command into your terminal from the
//! `workshop` directory:
//!
//! cargo run --bin control-flow

fn main() {
    println!("Enter a number:");
    let mut integer = utils::read_number();

    if integer == 42 {
        println!("42 is the meaning of life");
    }

    let abs = if integer < 0 {
        -integer
    } else {
        integer
    };

    println!("The absolute value of the number is {abs}");

    if integer > 0 {
        println!("The number is positive");

        // If `integer` is positive, let's
        // subtract one until `integer` is zero.
        while integer != 0 {
            println!("Count down: {integer}");
            integer -= 1;
        }
    } else if integer == 0 {
        println!("The number is zero");
    } else if integer < 0 {
        println!("The number is negative");

        // Challenge: if `integer` is negative,
        // add one until `integer` is zero,
        // printing "Count up: "

        // YOUR CODE HERE
        
    } else {
        println!("Whoops, mathematics broke");
    }
}
