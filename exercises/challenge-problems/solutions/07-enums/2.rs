enum Shape {
    Rectangle(u32, u32),
    Square(u32),
    Triangle(u32, u32),
}

fn area(shape: &Shape) -> u32 {
    match shape {
        Shape::Rectangle(w, h) => w * h,
        Shape::Square(side) => side * side,
        Shape::Triangle(base, height) => base * height / 2,
    }
}

fn main() {
    let shapes = vec![Shape::Rectangle(2, 3), Shape::Square(2), Shape::Triangle(4, 5)];
    let mut total = 0;
    for shape in &shapes {
        total += area(shape);
    }

    assert_eq!(total, 20);
    println!("Success!");
}
