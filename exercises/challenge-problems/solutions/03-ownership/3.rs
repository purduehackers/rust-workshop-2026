fn main() {
    let s1 = String::from("hello");
    let s2 = take_and_give_back(s1);
    println!("{s2}");
    println!("Success!");
}

fn take_and_give_back(s: String) -> String {
    println!("{s}");
    s
}
