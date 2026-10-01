// Implementing the `Display` trait lets a type be printed with {}.
// Fill in the blank.
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, __)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };

    assert_eq!(format!("{p}"), "(1, 2)");
    println!("Success!");
}
