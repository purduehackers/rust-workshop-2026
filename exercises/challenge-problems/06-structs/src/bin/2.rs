// Fix the error
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    let r = Rectangle { width: 3.0, height: 4.0 };
    r.width = 6.0;

    assert_eq!(r.width * r.height, 24.0);
    println!("Success!");
}
