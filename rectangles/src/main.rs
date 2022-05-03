#[derive(Debug)] 
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	// Rust knows the type of self is Rectangle due to this method's being inside the impl Rectangle context.
	// Methods can take ownership of self, borrow self immutably as we've done here, or borrow self mutably 
	// We'd use &mut self as the first parameter.
	fn area(&self) -> u32{
		self.width * self.height
	}

	fn can_hold(&self, other_rect: &Rectangle) -> bool {
		self.width > other_rect.width && self.height > other_rect.height	
	}
}

fn main() {
	let rect = Rectangle{
		width: 30,
		height: 50,
	};	
	
	println!("rect is {:#?}", rect);

	// Refactoring with Tuples
	// let rect = (30, 50);


	// We pass the Reference to the function parameter so that it can't take the ownership of the Rectangle
	println!("The area of the rectangle is {} square pixels.", rect.area());
	
	let rect1 = Rectangle{
		width: 30,
		height: 50,
	};	
	
	let rect2 = Rectangle{
		width: 10,
		height: 40,
	};	
	
	let rect3 = Rectangle{
		width: 60,
		height: 45,
	};	

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

/*
fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
	// dimensions.0 * dimensions.1
	// let (width, height) = dimensions;
	// width * height
} 
*/ 
