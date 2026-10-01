// Fill in the blanks
fn main() {
    let mut name = String::from("Ferris");
    shout(__ name);

    assert_eq!(name, "Ferris!");
    println!("Success!");
}

fn shout(s: __) {
    s.push('!');
}
