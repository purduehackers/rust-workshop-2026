struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    fn square(side: f64) -> Self {
        Self::new(side, side)
    }
}

fn main() {
    let r = Rectangle::new(3.0, 4.0);
    let s = Rectangle::square(2.0);

    assert_eq!(r.width * r.height, 12.0);
    assert_eq!(s.width, s.height);
    println!("Success!");
}
