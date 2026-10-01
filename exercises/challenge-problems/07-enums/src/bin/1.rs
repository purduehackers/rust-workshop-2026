// Fill in the blanks
enum Color {
    Red,
    Blue,
    Grayscale(f64),
}

fn brightness(color: &Color) -> f64 {
    match color {
        Color::Red => 0.3,
        Color::Blue => 0.1,
        Color::Grayscale(level) => __,
    }
}

fn main() {
    let colors = vec![Color::Red, Color::Blue, Color::__(0.6)];
    let mut total = 0.0;
    for color in &colors {
        total += brightness(color);
    }

    assert_eq!(total, 1.0);
    println!("Success!");
}
