fn main() {
    let mut s = String::from("hello");
    add_world(&mut s);

    assert_eq!(s, "hello world");
    println!("Success!");
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}
