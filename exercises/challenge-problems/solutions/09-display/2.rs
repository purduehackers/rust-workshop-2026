use std::fmt;

enum Color {
    Red,
    Blue,
    Grayscale(f64),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Blue => write!(f, "blue"),
            Color::Grayscale(level) => write!(f, "{}% gray", level * 100.0),
        }
    }
}

fn main() {
    assert_eq!(format!("{}", Color::Red), "red");
    assert_eq!(format!("{}", Color::Blue), "blue");
    assert_eq!(format!("{}", Color::Grayscale(0.5)), "50% gray");
    println!("Success!");
}
