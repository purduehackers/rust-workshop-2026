fn main() {
    let s = String::from("hello ");
    let mut s1 = s;
    s1.push_str("world");

    assert_eq!(s1, "hello world");
    println!("Success!");
}
