fn main() {
    let s = String::from("hello");
    borrow(&s);
    println!("{s}");
    println!("Success!");
}

fn borrow(s: &String) {
    println!("{s}");
}
