// Input output(IO) Library
// The IO library cames with the standard library (which is known as std)
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The Secret number is: {}", secret_number);

    println!("Please input the guess.");

    // Defining a mutable variable that means the variable can change
    // A String type is provided by the standard library that is growable
    // Growable means that we don't need to specify the size of the String
    // ::new line indicates that new is an associated function of the String type.
    // To summarize the line has created mutable variable that is currently bound to a new, empty instance of a String.
    let mut guess = String::new();

    // stdin can return a type and the type is io result
    // IO result can either be Ok or Err
    // When Err means the operation failed and contains information about how or why the operation failed.
    // and .expect while handle the error

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // String we saved the user's input in. The set of curly brackets {} is a placeholder

    println!("You guessed: {}", guess);
}
