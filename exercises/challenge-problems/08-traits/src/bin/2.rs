// A trait method with a body is a default. Implement only what is required.
trait Shape {
    fn area(&self) -> f64;

    fn describe(&self) -> String {
        format!("a shape with area {}", self.area())
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    __
}

fn main() {
    let s = Square { side: 3.0 };

    assert_eq!(s.describe(), "a shape with area 9");
    println!("Success!");
}
