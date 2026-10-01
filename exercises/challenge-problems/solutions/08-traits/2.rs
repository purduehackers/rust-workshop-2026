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
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let s = Square { side: 3.0 };

    assert_eq!(s.describe(), "a shape with area 9");
    println!("Success!");
}
