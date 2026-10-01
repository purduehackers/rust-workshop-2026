// `?` after a Result takes the value out of `Ok`, or returns the `Err`
// from the current function right away. Fill in the blanks. Don't use unwrap.
use std::num::ParseIntError;

fn add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a: i32 = a.parse()__;
    let b: i32 = b.parse()__;
    Ok(__)
}

fn main() {
    assert_eq!(add("1", "2"), Ok(3));
    assert!(add("1", "two").is_err());
    println!("Success!");
}
