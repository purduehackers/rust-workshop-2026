use std::collections::HashMap;

fn main() {
    let text = "the cat and the hat";
    let mut counts = HashMap::new();
    for word in text.split(' ') {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    assert_eq!(counts["the"], 2);
    assert_eq!(counts["cat"], 1);
    println!("Success!");
}
