use std::io;


fn main() {

	println!("What fibonacci number would you like?");

	let mut n = String::new();

	io::stdin()
	   .read_line(&mut n)
	   .expect("Failed to read line");

	let n: u32 = match n.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			panic!("Please input a valid positive number.");
		} 
	};

	let fibonacci_number = nth_fibonacci(n);
	println!("The {n} fibonacci number is {fibonacci_number}");
	
}


fn nth_fibonacci(n: u32) -> u32 {
	// 0 1 1 2 3 5 8 13 ....
	if n == 1 {
		return 1;
	} else if n == 0 {
		return 0;
	}
	return nth_fibonacci(n - 1) + nth_fibonacci(n - 2);
}
