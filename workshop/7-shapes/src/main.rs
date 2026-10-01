//! A program that does some simple computations
//! with shapes. Currently this compiles with a
//! stubbed solution, but we will live-code the
//! solution implementation together.
//!
//! Run this program by entering the following
//! command into your terminal from the
//! `workshop` directory:
//!
//! cargo run --bin shapes

// We import the solution for now to get rid of
// compiler errors
mod solution;
use solution::*;

fn main() {
    let red: Color = Color::Red;
    let blue: Color = Color::Blue;
    let gray: Color = Color::Grayscale(0.25);

    let rect1: Rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
        color: red,
    };
    let rect2: Rectangle = Rectangle {
        width: 2.0,
        height: 5.0,
        color: blue,
    };

    println!(
        "{} x {} = {}",
        rect1.width,
        rect1.height,
        rect1.area()
    );

    let rectangles: Vec<Rectangle> = vec![
        rect1,
        rect2,
        Rectangle {
            width: 3.0,
            height: 2.0,
            color: gray,
        },
    ];

    for rect in rectangles {
        rect.describe();
    }
}
