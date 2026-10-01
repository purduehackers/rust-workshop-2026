// Fix the error
enum Color {
    Red,
    Blue,
}

fn main() {
    let c = Color::Red;

    assert_eq!(c, Color::Red);
    assert!(c != Color::Blue);
    println!("Success!");
}
