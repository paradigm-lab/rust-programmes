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

    // Type inferences
    let z: f32 = p + y; // With explicit type Annotation
    println!("The value of z is: {}", z);

    // Numerical Operations
    // Addition
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;

    // Remainder
    let remainder = 43 % 5;

    println!(
        "Sum: {}, Subtraction: {}, Product: {}, Quotient: {}, 
    Reminder: {}",
        sum, difference, product, quotient, remainder
    );

    // Boolean Type (bool type)
    // 1 byte size
    let t = true;

    let f: bool = false; // With explicity type annotation
    println!("True: {}, False: {}", t, f);

    // Character type (char type)
    // 4 byte size
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';
    println!("Value c: {}, z {}, heart_eyed_cat {}", c, z, heart_eyed_cat);

    // Compound Types
    // Tuples and Arrays

    // Using destructure pattern matching to the tuple value
    // Taking a variable tup and bind it with a tuple
    let tup: (i32, f64, i32) = (500, 6.4, 1);
    // Using a pattern with let
    // And if the variable a not declared they will be infared
    // This is called destructuring, because it breaks the single tuple into three parts.
    let (xtup, ytup, ztup) = tup;

    println!(
        "The value of x is: {} The value of y is: {} The value of z is: {}",
        xtup, ytup, ztup
    );

    // Using a period(.) with the index of the value we want to access
    let tup_two: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup_two.0;
    let six_point_four = tup_two.1;
    let one = tup_two.2;

    println!(
        "The first index: {}, The second index: {}, The third index: {}",
        five_hundred, six_point_four, one
    );

    // The Array Type
    // Arrays can't grow (Fixed size)
    // An array is a single chunk of memory allocated on the stack.
    // Access element of an array using index.
    // Type annotation in arrays
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Creating an array that contains the same values for each element
    let _bunch_of_threes = [3; 12];

    // Accessing an array using the index
    let a = [1, 2, 3, 4, 5];
    let first_a = a[0];
    let second_b = a[1];

    println!(
        "The first index: {}, The second index: {}",
        first_a, second_b
    );

    // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];
    let index = 5 - 1;

    // The operation will panic at runtime
    // Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.
    let element = a[index];

    println!("The value of element is: {}", element);
}
