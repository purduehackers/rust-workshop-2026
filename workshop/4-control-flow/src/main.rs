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
    let mut int = utils::read_number();

    if int == 42 {
        println!("42 is the meaning of life");
    }

    let abs = if int < 0 {
        -int
    } else {
        int
    };

    println!("The absolute value of the number is {abs}");

    if int < 0 {
        println!("The number is negative");

        // Challenge: if `int` is negative,
        // add one until `int` is zero.

        // YOUR CODE HERE
 
    } else if int == 0 {
        println!("The number is zero");
    } else if int > 0 {
        println!("The number is positive");

        // If `int` is positive, let's
        // subtract one until `int` is zero.
        while int != 0 {
            println!("Count down: {int}");
            int -= 1;
        }
    } else {
        println!("Whoops, mathematics broke");
    }
}
