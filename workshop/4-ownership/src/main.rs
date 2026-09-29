//! A program that demonstrates a compiler error
//! as the result of violating the rules of ownership:
//! the program tries to access variable `a` after
//! its `String` value was moved into `a2`.
//!
//! Run this program by entering the following
//! command into your terminal from the
//! `workshop` directory:
//!
//! cargo run --bin ownership

fn main() {
    let a: String = utils::read_string();
    let a2: String = a;
    println!("{a} is awesome!");
}
