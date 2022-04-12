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
}

fn take_ownership(some_string: String) {
  println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
  println!("{some_integer}");
}
