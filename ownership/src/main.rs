fn main() {
    // Variable validation

    let _string_literal = "String Literal";

    // s is not valid here, it's not yet declared
    let mut s = String::from("Hello"); // s is valid from this point forward
                                       // Do stuff with s
    s.push_str(", Rustaceans"); // Appends a literal to a String
    println!("Our String is: {s}");
} // This scope is now over, and s is no longer valid
