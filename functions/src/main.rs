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

    println!("Is y greater than five? Answer: {}", greater_than_five(y));

    // The arms should return the same type base on the condition
    // Ternary Operation: The arm should be of the same type 
    // This helps to the compiler to be aware of the type at compile time rather than at the runtime
    let number = -5;
    let is_positive = if number > 0 { true } else { false };
    println!("The value of is_positive is: {}", is_positive);

    // A f(x) which doesn't return
    // It's is a statement
    divisible_by(16);
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

fn greater_than_five(x: i32) -> bool {
    // They are sametimes called "arms"
    // if statement use exprestions because NO semi-colon(;) and they also evalute to a value
    if x > 5 {
        true
    } else {
        false
    }
}

fn divisible_by(number: u32) {
    match number {
        divisible_by_four if divisible_by_four % 4 == 0 => println!("number is divisible by 4"),
        divisible_by_three if divisible_by_three % 3 == 0 => println!("number is divisible by 3"),
        divisible_by_two if divisible_by_two % 2 == 0 => println!("number is divisible by 2"),
        _ => println!("number is not divisible by 4, 3, or 2"),
    }
}
