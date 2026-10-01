use std::collections::HashMap;

fn main() {
    let name = String::from("alice");
    let mut ages = HashMap::new();
    ages.insert(name.clone(), 30);

    assert_eq!(ages[&name], 30);
    println!("Success!");
}
