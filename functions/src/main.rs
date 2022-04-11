fn main() {
    println!("Hello, Functions!");

    //  Expected an expresion found a statement
    // let x = (let y = 6);

    another_function(117, -117);
    expresion_example();
    let yimple = implicity_return();
    println!("Implicity Return from the function: {}", yimple);
    let y: i32 = -5;
    let xexpl = explicity_add_three(y);
    println!("The value of xexpl: {}", xexpl);
}

// Function parameter with the type signature
// We use comma to separate the parameter declarations
fn another_function(x: u32, y: i32) {
    println!("The value of x is: {}.", x);
    println!("The value of y is: {}.", y);
}

fn expresion_example() {
    let _x = 5;

    // Creating a block scope
    // This is an expression
    // Expression do not include ending semicolon
    // If we add a semicolon to the end of an expression, you turn it into a statement
    let y = {
        let x = 3;
        // This will be evaluted as an expression
        x + 1
    };

    println!("The value of y in expression_example f(x) is: {}", y);
}

fn implicity_return() -> i32 {
    // We should not use the semi-colon because this will change it to a statement
    // when we don't use semi-colon it will be an expression
    {
        let x: i32 = 5;
        x + 1
    }
}

fn explicity_add_three(x: i32) -> i32 {
    return x + 3;
}
