// Fix the errors
fn main() {
    let s = String::from("hello");
    add_world(s);

    assert_eq!(s, "hello world");
    println!("Success!");
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}
