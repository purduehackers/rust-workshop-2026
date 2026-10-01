// `"42".parse::<i32>()` returns a Result: `Ok(42)`, or `Err(..)` when the
// text is not a number. Fill in the blanks.
fn main() {
    let good: Result<i32, _> = "42".parse();
    let bad: Result<i32, _> = "4x2".parse();

    assert_eq!(good, __);
    assert!(bad.__());
    println!("Success!");
}
