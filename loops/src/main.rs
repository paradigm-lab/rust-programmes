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
