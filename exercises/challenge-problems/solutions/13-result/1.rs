fn main() {
    let good: Result<i32, _> = "42".parse();
    let bad: Result<i32, _> = "4x2".parse();

    assert_eq!(good, Ok(42));
    assert!(bad.is_err());
    println!("Success!");
}
