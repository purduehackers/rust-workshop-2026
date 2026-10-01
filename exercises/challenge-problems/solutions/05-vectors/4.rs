fn main() {
    let v = vec![3, 9, 2, 7];

    assert_eq!(largest(&v), 9);
    println!("Success!");
}

fn largest(v: &Vec<i32>) -> i32 {
    let mut max = v[0];
    for i in 1..v.len() {
        if v[i] > max {
            max = v[i];
        }
    }
    max
}
