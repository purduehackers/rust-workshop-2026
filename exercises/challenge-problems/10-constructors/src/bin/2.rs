// Don't modify main
struct Counter {
    count: u32,
}

impl Counter {
    __
}

fn main() {
    let mut c = Counter::new();
    c.increment();
    c.increment();

    assert_eq!(c.count, 2);
    println!("Success!");
}
