fn main() {
    let length: f64 = 4.5;
    let count: i32 = 3;
    let is_long: bool = length > 4.0;

    assert!(is_long);
    assert_eq!(count * 2, 6);
    println!("Success!");
}
