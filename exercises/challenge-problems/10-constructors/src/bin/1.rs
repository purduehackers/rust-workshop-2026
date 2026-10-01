// A function in an impl block without `self` is called with `Type::name(...)`.
// `Self` means the type being implemented. Fill in the blanks.
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> __ {
        __
    }

    fn square(side: f64) -> Self {
        Self::new(__, __)
    }
}

fn main() {
    let r = Rectangle::new(3.0, 4.0);
    let s = Rectangle::square(2.0);

    assert_eq!(r.width * r.height, 12.0);
    assert_eq!(s.width, s.height);
    println!("Success!");
}
