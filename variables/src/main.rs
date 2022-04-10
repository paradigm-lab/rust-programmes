fn main() {
    // Defining a mutable variable
    // Whereby mutable variables can be changed
    // While by default variable in rust are Immutable they can't be changed
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    // Using Shadowing
    let s = 5;
    let s = s + 1;
    let s = s * 2;
    println!("The Value of S is {}", s);

    // Making spaces and taking the size of the spaces
    let spaces = "  ";
    let spaces = spaces.len();
    println!("No of spaces: {}", spaces);

    // Rust Data types
    // we have got scalar types and Compound types
    // Scalar types are: Integer, Floating Point, Booleans and characters

    // Integer Types: Numbers without a fractional component
    // We have got Signed and Unsigned Integers

    // Floating Point: Fractional Numbers
    let p = 2.0; // F64
    let y: f32 = 3.0; // F32
    let z = p + y;
    println!("The value of z is: {}", z);
}
