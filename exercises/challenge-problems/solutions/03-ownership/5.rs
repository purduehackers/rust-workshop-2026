fn main() {
    let s: String = String::from("hello");
    let t: &str = &s;

    assert_eq!(s, t);
    println!("Success!");
}
