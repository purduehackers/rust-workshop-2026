// Only modify the println! line
struct Student {
    name: String,
    grade: u32,
}

fn main() {
    let s = Student { name: String::from("Ferris"), grade: 90 };
    let name = s.name;

    println!("{} got {}", s.name, s.grade);
    println!("Success!");
}
