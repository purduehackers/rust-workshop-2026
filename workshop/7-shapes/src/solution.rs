//! The solution code to make the main function in `main.rs`
//! compile.

// Suppress unused code warnings
#![allow(unused)]

// A Color enum. Enums in Rust represent data that can be
// one of a few possible values, or variants. In this case,
// the color can either be Blue, Red, and Grayscale with a
// decimal value from 0.0 to 1.0.
pub enum Color {
    Blue,
    Red,
    Grayscale(f64),
}

// A rectangle struct. Structs in Rust group multiple related
// values together, with each value stored in a named field.
// In this case, we declare a `Rectangle` with a width, height,
// and Color.
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

// A circle struct with a radius and Color.
pub struct Circle {
    pub radius: f64,
    pub color: Color,
}

// A Shape trait. Traits define shared behavior. In this case,
// the shared behavior is an `area` and `color` function along
// with a `NAME` constant. Implementing this trait automatically
// gives you access to the `describe` function.
pub trait Shape {
    const NAME: &str;

    fn area(&self) -> f64;
    fn color(&self) -> &Color;
    fn describe(&self) {
        print!("{}: area={}; color=", Self::NAME, self.area());
        match self.color() {
            Color::Blue => println!("blue"),
            Color::Red => println!("red"),
            Color::Grayscale(level) => println!("grayscale({level})"),
        }
    }
}

impl Shape for Rectangle {
    const NAME: &str = "Rectangle";

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

impl Shape for Circle {
    const NAME: &str = "Circle";

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

// A function that takes any two shapes as input and calls
// `describe` on the shape with the larger area.
pub fn describe_larger<A: Shape, B: Shape>(a: &A, b: &B) {
    println!("The larger shape is:");
    if a.area() > b.area() {
        a.describe();
    } else {
        b.describe();
    }
}
