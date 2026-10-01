// Fill in the blanks
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(__) -> f64 {
        self.width * self.height
    }

    fn scale(__, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let __ r = Rectangle { width: 3.0, height: 4.0 };
    r.scale(2.0);

    assert_eq!(r.area(), 48.0);
    println!("Success!");
}
