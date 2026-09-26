#![allow(clippy::vec_init_then_push)]

//! The walk-through program.
//!
//! Run this program by entering the following
//! command into your terminal from the
//! `workshop` directory:
//!
//! cargo run --bin rectangle

// We import the solution for now to get rid of
// compiler errors
mod solution;
use solution::*;

fn main() {
    let gray = Color::Grayscale(0.25);
    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
        color: Color::Red,
    };
    println!("{} x {} = {}", rect.width, rect.height, rect.area());

    let mut rects: Vec<Rectangle> = Vec::new();
    rects.push(rect);
    rects.push(Rectangle {
        width: 1.0,
        height: 1.0,
        color: gray,
    });
    rects.push(Rectangle {
        width: 2.0,
        height: 5.0,
        color: Color::Blue,
    });

    let mut total_area = 0.0;
    for rect in rects {
        rect.describe();
        total_area += rect.area();
    }

    // Challenge: print how many rectangles there are,
    // and the total area. (Hint: use `rects.len()`)

    // YOUR CODE HERE
    
}
