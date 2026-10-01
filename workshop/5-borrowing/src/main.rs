//! A program that demonstrates a compiler error
//! as the result of violating the rules of ownership:
//! the program tries to access variable `a` after
//! its `String` value was moved into `a2`.
//!
//! Run this program by entering the following
//! command into your terminal from the
//! `workshop` directory:
//!
//! cargo run --bin borrowing

fn main() {
    let mut a: String = utils::read_string();

    let b: &String = &a;
    let c: &String = &a;
    let d: &mut String = &mut a;

    // ... a bunch of complicated code

    println!("{b} {c} {d}");
}
