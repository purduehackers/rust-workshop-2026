//! A program that demonstrates mutability,
//! scopes, and data types.
//!
//! Run this program by entering the following
//! command into your terminal from the
//! `workshop` directory:
//!
//! cargo run --bin variables

fn main() {
    let power = 1000;
    println!("The power level is: {power}");

    // Try to uncomment this below:

    // power = 2000;
    // println!("The power level is now: {power}");
}

fn data_types() {
    // Because the below variables are unused, Rust
    // will throw a warning (good!). We silence the
    // warning, but you can comment this out to see
    // what the warnings look like.
    #![allow(unused)]

    // `int` is the integer type `i32`
    let mut int = 5;
    // `int2` is annotated explicitly
    let int2: i32 = 5;
    let true_or_false: bool = true;
    let decimal: f64 = 4.5;

    // Try to uncomment this below:

    // int = decimal;
}
