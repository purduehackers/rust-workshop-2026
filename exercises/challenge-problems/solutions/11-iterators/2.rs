fn main() {
    let v = vec![3, 8, 5, 12, 7];
    let evens: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
    assert_eq!(evens, vec![8, 12]);

    let words = vec!["apple", "fig", "banana"];
    let long: Vec<&str> = words.into_iter().filter(|w| w.len() > 3).collect();
    assert_eq!(long, vec!["apple", "banana"]);

    println!("Success!");
}
