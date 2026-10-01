// Fix the error with the smallest change. Don't delete any line.
use std::collections::HashMap;

fn main() {
    let name = String::from("alice");
    let mut ages = HashMap::new();
    ages.insert(name, 30);

    assert_eq!(ages[&name], 30);
    println!("Success!");
}
