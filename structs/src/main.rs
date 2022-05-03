fn main() {
	struct User {
		// We define the names and types of the pieces of data, which we call fields.
		username: String,
		email: String,
		sign_in_count: u64,
		active: bool,	
	}
	
	let mut user1 = User {
		email: String::from("younggods117@twitch.tv"),
		username: String::from("youvnggod"),
		active: true,
		sign_in_count: 1,	
	};

	fn build_user(email: String, username: String) -> User {
		User {	
			email,
			username,
			active: true,
			sign_in_count: 1,	
		}

	}

	// We can use dot notation to get a specific value from a struct
	println!("User's email is {}", user1.email);
	
	user1.username = String::from("Paradigm");
	println!("User's email is {}", user1.username);
	
	let user2 = build_user(String::from("test@testington.com"), String::from("testTestingsworth"));
	println!("User2 email: {}, username: {}", user2.email, user2.username);


	// Creating Instance From other Instance with Struct Update Syntax
	let inactive_user2 = User {
		active: false,
		..user2
	};
	
	println!("Inactive User2 Info - active: {}, username: {}", inactive_user2.active, inactive_user2.username);
}


