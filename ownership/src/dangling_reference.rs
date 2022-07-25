fn main() {
    // let reference_to_nothing = dangle();
    let no_dangling = no_dangle();
    println!("No Dangling Pointer: {no_dangling}");
}

/*
// Not going to work as 's' falls out of scope and
// &s point to nothing
fn dangle() -> &String {
  let s = String::from("Hello, Rustaceans");
  &s
}
*/

// Works because we actually return the String
// This works without any problems. Ownership is moved out, and nothing is deallocated.
fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}

/*
    Not going to work as s falls out of scope and,
    &s points to nothing
fn no_dangle() -> &String {
    let s = String::from("Hello");
    &s
}
*/

/*
    Rules of References
    -> Let's recap what we've discussed about references:
        -> At any given time, you can have either one mutable reference or any number of immutable references.
        -> References must always be valid.
*/
