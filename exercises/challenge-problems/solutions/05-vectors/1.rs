fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    let mut sum = 0;
    for x in &v {
        sum += x;
    }

    assert_eq!(sum, 6);
    assert_eq!(v.len(), 3);
    println!("Success!");
}
