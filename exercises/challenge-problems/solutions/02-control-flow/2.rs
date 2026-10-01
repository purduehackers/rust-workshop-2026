fn main() {
    let n = 7;
    let parity = if n % 2 == 0 { "even" } else { "odd" };

    assert_eq!(parity, "odd");
    println!("Success!");
}
