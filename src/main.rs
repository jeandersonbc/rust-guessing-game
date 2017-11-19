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

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
