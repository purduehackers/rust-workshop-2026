// Fill in the blanks
fn main() {
    let v: __ = vec![1, 2, 3];
    let mut sum = 0;
    for x in __ {
        sum += x;
    }

    assert_eq!(sum, 6);
    assert_eq!(v.len(), 3);
    println!("Success!");
}
