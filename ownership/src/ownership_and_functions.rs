fn main() {
  // It has moved
  let s = String::from("Hello"); // S comes into scope.

  // s' value moves into the function (Implements move)
  take_ownership(s);
  // and is no longer valid here.

  // It made copy (Copy)
  let x = 5; // x comes into scope

  // x would move into the function
  makes_copy(x);
  // But i32 is copy (meaning has trait copy), So
  // It's okay to still use x afterward.

  println!("x is now: {x}");

  // println!("s is now: {s}");

  let s1 = gives_ownership();
  // Moved s2 to s3
  let s2 = String::from("Hello Rustaceans");
  let s3 = takes_and_gives_back(s2);

  println!("s1: {s1}, s3: {s3}");

  let (s4, len) = calculate_length(s3);
  println!("The length of '{}' is: {}", s4, len);
}

fn take_ownership(some_string: String) {
  println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
  println!("{some_integer}");
}

// This function will move its return values into the
// function that calls it
fn gives_ownership() -> String {
  // some_string comes into scope
  let some_string = String::from("hello");

  // some_string is returned and moves out to the
  // calling function
  some_string
}

// Will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
  //a_string is returned and moves out to the calling function.
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len() returns the length of a String

  (s, length)
}
