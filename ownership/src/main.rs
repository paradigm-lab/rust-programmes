fn main() {
    // Variable validation

    let _string_literal = "String Literal";

    // s is not valid here, it's not yet declared
    let mut s = String::from("Hello"); // s is valid from this point forward
                                       // Do stuff with s
    s.push_str(", Rustaceans"); // Appends a literal to a String
    println!("Our String is: {s}");

    /*
    Integers are simple values and stored
    in the stack so in this case y is the copy of 5.
    */
    let x = 5;
    let y = x;
} // This scope is now over, and s is no longer valid
