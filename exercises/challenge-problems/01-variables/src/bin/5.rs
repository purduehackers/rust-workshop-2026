// Fix the error. Rust will not add an integer and a decimal.
fn main() {
    let integer = 5;
    let decimal = 4.5;
    let sum = integer + decimal;

    assert_eq!(sum, 9.5);
    println!("Success!");
}
