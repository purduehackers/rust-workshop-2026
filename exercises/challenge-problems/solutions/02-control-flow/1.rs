fn main() {
    let n = -7;
    let sign = if n < 0 {
        -1
    } else if n == 0 {
        0
    } else {
        1
    };

    assert_eq!(sign, -1);
    println!("Success!");
}
