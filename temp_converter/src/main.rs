use std::io;

fn main() {
	println!("Please input a temperature...");

	let mut temperature = String::new();

	io::stdin()
		.read_line(&mut temperature)
		.expect("Failed to read line");

	let temperature = temperature.trim();

	let celsius = get_celsius_temperature_from_string(temperature);
	let fahrenheit = get_fahrenheit_temperature_from_string(temperature);

	if celsius != "" {
		let celsius = match celsius.parse() {
			Ok(num) => num,
			Err(_) => {
				panic!("Please enter a valid temperature");
			}
		};

		println!(
			"{} is equal to {}F",
			temperature,
			celcius_to_fahrenheit(celsius)
		);
	} else if fahrenheit != "" {
		let fahrenheit = match fahrenheit.parse() {
			Ok(num) => num,
			Err(_) => {
				panic!("Please enter a valid temperature");
			}
		};

		println!(
			"{} is equal to {}C",
			temperature,
			fahrenheit_to_celcius(fahrenheit)
		);
	} else {
		panic!("Please enter a valid temperature");
	}

	println!("You input: {temperature}");

	/*
	let celcius: f64 = fahrenheit_to_celcius(fahrenheit);
	println!("{fahrenheit}F is equal to {celcius}");
	*/
}

fn get_temperature_from_string(temp_string: &str) -> (f64, &str) {
	let temp_string_bytes = temp_string.as_bytes();
	let mut temperature: &str = "";
	let mut unit = "fahrenheit";

	for (i, &item) in temp_string_bytes.iter().enumerate() {
		// Using the byte literals to check for the character in the bytes
		if item == b'F' || item == b'f' {
			temperature = &temp_string[0..i];
			break;
		} else if item == b'C' || item == b'c' {
			temperature = &temp_string[0..i];
			unit = "celcius";
			break;
		}
	}
	let temperature: f64 = match temperature.parse() {
		Ok(num) => num,
		Err(_) => {
			panic!("Invalid temperature!")
		}
	};
	return (temperature, unit);
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
	(f - 32.0) * (5.0 / 9.0)
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
	(c * 9.0) / 5.0 + 32.0
}
