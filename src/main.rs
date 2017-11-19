use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please, input your guess.");

    // "let" statements allow to create variables...
    // In Rust, variables are IMMUTABLE by default! To make the variable
    // multable, you have to add the modifier "mut", as well.
    // Notice that, if you try to change the value of an immutable variable,
    // the Rust compiler will complain about it.
    let mut guess = String::new();

    // The "&" operator refers to the memory address of the variable "guess".
    // Again, by default, references are immutable. In this case, we need to
    // make the reference mutable in other get the input.
    //
    // Another curious thing here is the ".expect" function. The function
    // "read_line" returns a "Result" which indicates if the function was
    // successfully executed. The Result type is responsible for encapsulating
    // error-handling and the Rust compiler will emit a warning whenever you
    // forget to use the return value of a function that you are supposed to be
    // using. In this case, if something bad occurs, the execution halts and
    // the informed message will be displayed.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
