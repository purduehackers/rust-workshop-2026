trait Describe {
    fn describe(&self) -> String;
}

struct Circle {
    radius: f64,
}

impl Describe for Circle {
    fn describe(&self) -> String {
        format!("a circle of radius {}", self.radius)
    }
}

fn main() {
    let c = Circle { radius: 2.0 };

    assert_eq!(c.describe(), "a circle of radius 2");
    println!("Success!");
}
