fn main() {
    // Repeting code with loop
    /*
    loop {
        println!("Again!");
    }
    */

    // Return values from loops
    let value = count_to_ten();
    println!("The result is: {}", value);

    // Conditional Loops with while
    countdown_from(5);
}

fn count_to_ten() -> u32 {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    result
}

fn countdown_from(x: u32) {
    let mut number = x;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
}
