// Rewrite total_len with iter, map and sum. Don't modify main.
fn total_len(words: &Vec<&str>) -> usize {
    let mut total = 0;
    for w in words {
        total += w.len();
    }
    total
}

fn main() {
    let words = vec!["one", "three", "five"];

    assert_eq!(total_len(&words), 12);
    println!("Success!");
}
