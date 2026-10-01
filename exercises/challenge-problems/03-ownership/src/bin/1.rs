// Make it compile without deleting anything
fn main() {
    let x = String::from("hello");
    let y = x;
    println!("{x}, {y}");
    println!("Success!");
}
