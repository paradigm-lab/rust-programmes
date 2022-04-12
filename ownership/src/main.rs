fn main() {
    // Variable validation
    // s is not valid here, it's not yet declared
    let s = String::from("hello"); // s is valid from this point forward
                                   // Do stuff with s
    println!("Our String is: {s}")
} // This scope is now over, and s is no longer valid
