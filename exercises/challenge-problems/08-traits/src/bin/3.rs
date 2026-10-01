// Fill in the blanks
trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn larger_area<T: __, U: __>(a: &T, b: &U) -> f64 {
    if a.area() > b.area() { a.area() } else { b.area() }
}

fn main() {
    let s = Square { side: 3.0 };
    let r = Rectangle { width: 2.0, height: 6.0 };

    assert_eq!(larger_area(&s, &r), 12.0);
    println!("Success!");
}
