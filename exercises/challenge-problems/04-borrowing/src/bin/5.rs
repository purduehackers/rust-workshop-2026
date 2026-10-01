// Move one line so it compiles. Don't delete anything.
fn main() {
    let mut a = String::from("hello");
    let b = &a;
    let c = &a;
    let d = &mut a;
    d.push_str(" world");
    println!("{b} {c}");
    println!("{d}");
    println!("Success!");
}
