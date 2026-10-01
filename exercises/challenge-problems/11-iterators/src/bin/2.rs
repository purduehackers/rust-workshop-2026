// `filter` keeps the items for which the closure returns true.
// Fill in the blanks.
fn main() {
    let v = vec![3, 8, 5, 12, 7];
    let evens: Vec<i32> = v.into_iter().filter(|x| __).collect();
    assert_eq!(evens, vec![8, 12]);

    let words = vec!["apple", "fig", "banana"];
    let long: Vec<&str> = words.into_iter().filter(|w| __).collect();
    assert_eq!(long, vec!["apple", "banana"]);

    println!("Success!");
}
