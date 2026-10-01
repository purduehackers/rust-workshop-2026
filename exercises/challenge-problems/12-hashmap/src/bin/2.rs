// `entry(key).or_insert(value)` inserts `value` if `key` is missing, then
// returns a mutable reference to whatever is stored. Fill in the blanks.
use std::collections::HashMap;

fn main() {
    let text = "the cat and the hat";
    let mut counts = HashMap::new();
    for word in text.split(' ') {
        let count = counts.entry(word).or_insert(__);
        *count += __;
    }

    assert_eq!(counts["the"], 2);
    assert_eq!(counts["cat"], 1);
    println!("Success!");
}
