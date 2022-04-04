use std::fmt::format;

// Created a new function
// Passing a 64 bit signed integer type in the function (signed and unsigned)
// Using a formated string to print the number in the parameter
fn greet(x: &str) -> String {
    format!("Hello to {}", x)
}

fn main() {
    // Change the String into a Capital String
    // String is located at the Heap
    // &str Going to the actuall address
    let name: &str = "Rust";
    let result = greet(name);
    println!("My greeting: {}", result);
}
