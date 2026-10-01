fn main() {
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }

    assert_eq!(sum, 5050);
    println!("Success!");
}
