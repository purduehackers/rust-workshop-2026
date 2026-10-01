struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut c = Counter::new();
    c.increment();
    c.increment();

    assert_eq!(c.count, 2);
    println!("Success!");
}
