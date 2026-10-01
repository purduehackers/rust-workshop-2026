// This code has no errors. Why is `s.push_str` allowed while `r` exists?
fn main() {
    let mut s = String::from("hello");
    let r = &s;
    println!("{r}");
    s.push_str(" world");

    assert_eq!(s, "hello world");
    println!("Success!");
}
