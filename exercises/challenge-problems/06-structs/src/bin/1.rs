// Fix the error
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    let r = Rectangle { width: 3.0 };

    assert_eq!(r.width * r.height, 12.0);
    println!("Success!");
}
