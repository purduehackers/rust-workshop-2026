use std::num::ParseIntError;

fn add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a: i32 = a.parse()?;
    let b: i32 = b.parse()?;
    Ok(a + b)
}

fn main() {
    assert_eq!(add("1", "2"), Ok(3));
    assert!(add("1", "two").is_err());
    println!("Success!");
}
