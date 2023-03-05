// Use Standard input module?
// io = input/output library
// std = standard library
use std::io;
// Rng trait defines methods that random number generatoris implement
use rand::Rng;
// Allows ability to take in user input

fn main() {
    // UI
    println!("\n <<< Guess the number! >>> \n");
    let secret_number = rand::thread_rng().gen_range(1..=50);
    println!("\n <<< The Secrect Number Is: {secret_number} >>> \n");

    println!("\n <<< Please input your guess. >>> \n");

    let mut guess = String::new();
    // Store guess in a mutable variable with type string
    let apples = 34;
    // in rust, by default, variables CANNOT change. They are immutable

    // Calling stdin function which will allow user input and handle errors
    io::stdin()
        // call read line method on stdin
        // pass mut guess
        // & = this argument is a reference
        .read_line(&mut guess)
        // why do we need to pass &mut to guess when it's already declared above in a variable?
        // read_line puts everything the user enters into a string but ALSO RETURNS A RESULT VALUE
        .expect("Failed to read line");
        // Result = enum
        // Result = Ok or Err

    println!("\n You guessed:\n {guess}");
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);


}
