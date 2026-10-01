// Fix the error without deleting any line
fn main() {
    let s = String::from("hello");
    print_str(s);
    println!("{s}");
    println!("Success!");
}

fn print_str(s: String) {
    println!("{s}");
}
