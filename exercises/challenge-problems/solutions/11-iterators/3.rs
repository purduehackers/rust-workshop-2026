fn total_len(words: &Vec<&str>) -> usize {
    words.iter().map(|w| w.len()).sum()
}

fn main() {
    let words = vec!["one", "three", "five"];

    assert_eq!(total_len(&words), 12);
    println!("Success!");
}
