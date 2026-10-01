fn main() {
    let mut name = String::from("Ferris");
    shout(&mut name);

    assert_eq!(name, "Ferris!");
    println!("Success!");
}

fn shout(s: &mut String) {
    s.push('!');
}
