// `#[derive(..)]` asks the compiler to write trait impls for you:
// Debug for {:?}, PartialEq for ==, Clone for .clone().
// Fix the errors by adding one line.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();

    assert_eq!(a, b);
    println!("{a:?}");
    println!("Success!");
}
