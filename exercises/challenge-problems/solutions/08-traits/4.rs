trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn total_area<T: Shape>(shapes: &Vec<T>) -> f64 {
    let mut total = 0.0;
    for shape in shapes {
        total += shape.area();
    }
    total
}

fn main() {
    let rects = vec![
        Rectangle { width: 1.0, height: 2.0 },
        Rectangle { width: 3.0, height: 4.0 },
    ];

    assert_eq!(total_area(&rects), 14.0);
    println!("Success!");
}
