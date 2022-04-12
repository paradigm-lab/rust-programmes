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
    println!("{y}");
    // Immutable String
    /*
        A pointer holds the contents of the String,
        A length -> How much memory (In bytes)
        A capacity -> Total amount of memory (In bytes)
    */
    // We are not copying hello but
    // We are copying the data from the stack

    // This means s1 is no longer existing
    // This is an example of a "move"
    // let s1 = String::from("Hello");
    // let s2 = s1;

    // This is a deep copy and it mantain the value
    // This is an example of a "Clone"
    let c1 = String::from("Hello");
    let c2 = c1.clone();

    println!("c1: {c1}, c2: {c2}");
} // This scope is now over, and s is no longer valid
