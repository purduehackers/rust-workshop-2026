fn main() {
    let names = vec![String::from("a"), String::from("b")];
    for name in &names {
        println!("{name}");
    }

    assert_eq!(names.len(), 2);
    println!("Success!");
}
