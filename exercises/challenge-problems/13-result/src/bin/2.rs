// Fill in the blanks
fn main() {
    let inputs = vec!["3", "x", "5"];
    let mut total = 0;
    let mut errors = 0;
    for s in inputs {
        match s.parse::<i32>() {
            __ => total += n,
            __ => errors += 1,
        }
    }

    assert_eq!(total, 8);
    assert_eq!(errors, 1);
    println!("Success!");
}
