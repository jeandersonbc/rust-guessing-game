// Indicates that this an external lib.
// This is equivalent to "use rand".
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    loop {
        println!("Please, input your guess (between 1 and 100).");

        // "let" statements allow to create variables...
        // In Rust, variables are IMMUTABLE by default! To make the variable
        // mutable, you have to add the modifier "mut", as well.
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

        // It is necessary to convert the string guess to an integer type
        // Here, we are shadowing the previous guess declaration and also
        // annotating it as a unsigned 32-bit integer.
        //
        // Replacing a call to "expect" by a match expression is the proper
        // way to handle errors in this situation. Since "parse" returns a
        // result, we can use pattern matching to handle problems or continue
        // the execution if everything is fine
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Bad input. Only integers are allowed!");
                continue;
            }
        };

        // Match expressions are similar to switch-case in Java
        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
