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

    let spaces = "  ";
    let spaces = spaces.len();
    println!("No of spaces: {}", spaces);
}
