fn main() {
    let mut x = utils::read_number();

    if x == 42 {
        println!("42 is the meaning of life");
    }

    let abs_x = if x < 0 {
        -x
    } else {
        x
    };

    println!("The absolute value of the number is {abs_x}");

    if x < 0 {
        println!("The number is negative");
    } else if x == 0 {
        println!("The number is zero");
    } else if x > 0 {
        println!("The number is positive");

        // If `x` is positive, let's
        // subtract one until `x` is zero.
        while x != 0 {
            println!("Count down: {x}");
            x -= 1;
        }
    } else {
        println!("Whoops, mathematics broke");
    }
}
