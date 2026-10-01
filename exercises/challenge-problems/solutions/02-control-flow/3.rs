fn main() {
    let mut n = 10;
    let mut sum = 0;
    while n > 0 {
        sum += n;
        n -= 1;
    }

    assert_eq!(sum, 55);
    assert_eq!(n, 0);
    println!("Success!");
}
