// Make the necessary variable mutable
fn main() {
    let s = String::from("hello ");
    let s1 = s;
    s1.push_str("world");

    assert_eq!(s1, "hello world");
    println!("Success!");
}
