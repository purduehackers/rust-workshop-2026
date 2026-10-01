use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("alice", 90);
    scores.insert("bob", 72);

    assert_eq!(scores.get("alice"), Some(&90));
    assert_eq!(scores.get("carol"), None);
    assert_eq!(scores.len(), 2);
    println!("Success!");
}
