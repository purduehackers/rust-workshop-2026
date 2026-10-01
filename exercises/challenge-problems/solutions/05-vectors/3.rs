fn main() {
    let mut squares = Vec::new();
    for i in 1..=5 {
        squares.push(i * i);
    }

    assert_eq!(squares, vec![1, 4, 9, 16, 25]);
    assert_eq!(squares[3], 16);
    println!("Success!");
}
