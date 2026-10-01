// Fix the error
fn main() {
    let n = 7;
    let parity = if n % 2 == 0 { "even" } else { 1 };

    assert_eq!(parity, "odd");
    println!("Success!");
}
