// A HashMap stores key -> value pairs. `get` returns an Option holding a
// reference to the value. Fill in the blanks.
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("alice", 90);
    scores.insert("bob", 72);

    assert_eq!(scores.get("alice"), Some(__));
    assert_eq!(scores.get("carol"), __);
    assert_eq!(scores.len(), __);
    println!("Success!");
}
