// `Option<i32>` is an enum with two variants: `Some(value)` and `None`.
// Fill in the blanks.
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    assert_eq!(six, Some(6));
    assert_eq!(none, __);
    println!("Success!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        __ => None,
        __ => Some(i + 1),
    }
}
