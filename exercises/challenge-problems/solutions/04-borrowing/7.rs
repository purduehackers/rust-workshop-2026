// `r` is last used in the println!, so its borrow ends there.
fn main() {
    let mut s = String::from("hello");
    let r = &s;
    println!("{r}");
    s.push_str(" world");

    assert_eq!(s, "hello world");
    println!("Success!");
}
