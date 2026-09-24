/// Reads one line of user input.
///
/// # Panics
///
/// This function panics and terminates the program
/// if the program cannot read user input for
/// whatever reason.
pub fn read_string() -> String {
    // We will read the user input into a string.
    // First, we declare an empty string to read the user input into.
    let mut user_input_string = String::new();

    // Read the actual user input into the string.
    std::io::stdin()
        .read_line(&mut user_input_string)
        .expect("Could not read line.");
    user_input_string
}

/// Reads a number from user input.
///
/// # Panics
///
/// This function panics and terminates the program
/// if the user input is not a valid number, or the
/// program cannot read user input for whatever reason.
pub fn read_number() -> i32 {
    // We will read the user input into a string.
    // First, we declare an empty string to read the user input into.
    let mut user_input_string = String::new();

    // Read the actual user input into the string.
    std::io::stdin()
        .read_line(&mut user_input_string)
        .expect("Could not read line.");

    // We convert that string by parsing it into an i32,
    // panicking if the user input cannot be parsed as an i32.
    let number: i32 = user_input_string
        .trim()
        .parse()
        .expect("Input was not a number.");
    number
}
