// Fix the error, then fill in the blank
fn main() {
    let squares = Vec::new();
    for i in 1..=5 {
        squares.push(i * i);
    }

    assert_eq!(squares, vec![1, 4, 9, 16, 25]);
    assert_eq!(squares[__], 16);
    println!("Success!");
}
