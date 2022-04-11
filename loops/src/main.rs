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

    // Iteration over a collection in an array
    countdown_to_five();

    // Count down from five using a range and reverse method
    countdown_from_five();
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
    println!("LIFTOFF!!!");
}

fn countdown_to_five() {
    let numbers = [1, 2, 3, 4, 5];

    // This will increase the safety of the code and eliminated the bugs
    // That might result from going beyond the end of the array,
    // Or not going far enough and missing some items
    // Safety and conciseness of the for loops
    for number in numbers.iter() {
        println!("{}", number);
    }
    println!("LIFTOFF!");
}

fn countdown_from_five() {
    // first bound inclusive and last bound exclusive
    for number in (1..6).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!");
}
