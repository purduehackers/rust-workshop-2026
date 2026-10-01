fn main() {
    let mut a = String::from("hello");
    let b = &a;
    let c = &a;
    println!("{b} {c}");
    let d = &mut a;
    d.push_str(" world");
    println!("{d}");
    println!("Success!");
}
