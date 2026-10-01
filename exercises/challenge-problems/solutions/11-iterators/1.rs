fn main() {
    let v = vec![1, 2, 3, 4];

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8]);

    let total: i32 = v.iter().sum();
    assert_eq!(total, 10);

    println!("Success!");
}
