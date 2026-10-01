// `v.iter()` visits each item by reference. `map` transforms every item
// with a closure, `sum` adds them up, `collect` builds a new Vec.
// Fill in the blanks.
fn main() {
    let v = vec![1, 2, 3, 4];

    let doubled: Vec<i32> = v.iter().map(|x| __).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8]);

    let total: i32 = v.iter().__();
    assert_eq!(total, 10);

    println!("Success!");
}
