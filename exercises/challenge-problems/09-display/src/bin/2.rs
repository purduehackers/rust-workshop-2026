// Fill in the impl
use std::fmt;

enum Color {
    Red,
    Blue,
    Grayscale(f64),
}

impl fmt::Display for Color {
    __
}

fn main() {
    assert_eq!(format!("{}", Color::Red), "red");
    assert_eq!(format!("{}", Color::Blue), "blue");
    assert_eq!(format!("{}", Color::Grayscale(0.5)), "50% gray");
    println!("Success!");
}
