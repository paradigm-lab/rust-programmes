fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("The value of s is: {s}");

    /*
    ->  Data race is similar to a race condition and happens when these three behavious occur
          Two or more pointers access the same data at the same time.
          At least one of the pointers is being used to write to the data.
          There's no mechanism being used to synchronize access to the data.
    */

    /*
      To fix this we should have only one mutable referrence at a time
    */
    {
        let r1 = &mut s;
        println!("r1: {r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("r2: {r2}");

    /*
    {
        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s;

        println!("r1: {r1}, r2: {r2}, r3: {r3}");
    }
    */

    {
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("r1: {r1} and r2: {r2}");
        // r1 and r2 are no longer used after this point

        let r3 = &mut s; // no problem
        println!("r3: {r3}");
    }
}

fn change(some_string: &mut String) {
    some_string.push_str(", Rustaceans");
}
