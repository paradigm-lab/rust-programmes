// Created a new function
// Passing a 64 bit signed integer type in the function
// Using a formated string to print the number in the parameter
fn greet(x: String) {
    println!("Hello to {}", x);
}

fn main() {
    // Change the String into a Capital String
    // String is located at the Heap
    let my_greeting = "Hello, world".to_string();

    greet(my_greeting);
}
