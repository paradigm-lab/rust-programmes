struct Employee {
    name: String,
    id: i64,
}

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

    let employee = Employee {
        name: "Stream".to_string(),
        id: 101,
    };

    // let alt = String::from("hello");

    // Rust can't print the whole structure
    // println!("Struct {}", employee);

    println!("Name: {}", employee.name);
    println!("Id: {}", employee.id);
}
