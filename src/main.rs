// Rng trait defines methods that random number generatoris implement
use rand::Rng;
// Allows ability to take in user input

use std::cmp::Ordering;
// io = input/output library
// std = standard library
use std::io;

fn main() {
    // UI
    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("\n <<< Guess the number! >>> \n");

//    println!("\n <<< The Secrect Number Is: {secret_number} >>> \n");


   loop {
        let mut guess = String::new();
        // Store guess in a mutable variable with type string
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
        let guess: u32 = match guess.trim().parse() {
            // Allows user to keep guessing if not typed number instead of crashing game.
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("\n You guessed:\n {guess}");
    
 
        match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small! TRY AGAIN!"),
        Ordering::Greater => println!("Too Large! TRY AGAIN!"),
        Ordering::Equal => {
                println!("YOU WIN!!!!!!!!!");
                break; 
            }
    }}


}
