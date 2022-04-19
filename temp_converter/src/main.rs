use std::io;

fn main() {

	println!("Please input a temperature in Fahrenheit.");

	let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                panic!("Please input a number.");
            }
        };

}


fn fahrenheit_to_celcius(f: f64) -> f64 {
	(f - 32.0) * (5.0 / 9.0)
}


fn celcius_to_fahrenheit(c: f64) -> f64 {
	(c * 9.0) / 5.0 + 32.0
}
