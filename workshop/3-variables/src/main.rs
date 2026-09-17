fn main() {
    let power = 5000;
    println!("The power level is: {power}");

    {
        let power = power * 2;
        println!("The power level is: {power}. The power level is over 9000!!");
    }

    println!("The power level is back to: {power}");

    // Try to uncomment this below:

    // power = 1000;
    // println!("The power level is: {power}");
}

// Silence the "unused function" warning
#[allow(unused)]
fn data_types() {
    let a: i32 = 5;
    let b: bool = true;
    let c = false;
    
    let mut d: f64 = 2.5;
    
    // Try to uncomment this below:
    // d = a;
    
    // Then, try to uncomment this:
    // let d = a;
}
