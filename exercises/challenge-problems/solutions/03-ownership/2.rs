fn main() {
    let a = 5;
    let b = a;
    println!("{a} {b}");

    let s = String::from("five");
    let t = &s;
    println!("{s} {t}");

    println!("Success!");
}
