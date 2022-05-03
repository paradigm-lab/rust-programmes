#[derive(Debug)] 
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let rect1 = Rectangle{
		width: 30,
		height: 50,
	};	
	
	println!("rect1 is {:#?}", rect1);

	// Refactoring with Tuples
	// let rect1 = (30, 50);


	// We pass the Reference to the function parameter so that it can't take the ownership of the Rectangle
	println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
	// dimensions.0 * dimensions.1
	// let (width, height) = dimensions;
	// width * height
}  
