// Fix the errors
fn main() {
    let s: String = "hello";
    let t: &str = s;

    assert_eq!(s, t);
    println!("Success!");
}
