struct User {
    // We define the names and types of the pieces of data, which we call fields.
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("younggods117@twitch.tv"),
        username: String::from("youvnggod"),
        active: true,
        sign_in_count: 1,
    };

    // We can use dot notation to get a specific value from a struct
    println!("User's email is {}", user1.email);
    user1.username = String::from("Paradigm");
    println!(
        "User {} has signed in {}",
        user1.username, user1.sign_in_count
    );

    let user2 = build_user(
        String::from("test@testington.com"),
        String::from("testTestingsworth"),
    );
    println!("User2 email: {}, username: {}", user2.email, user2.username);

    // Creating Instance From other Instance with Struct Update Syntax
    let inactive_user2 = User {
        active: false,
        ..user2
    };

    println!(
        "Inactive User2 Info - active: {}, username: {}",
        inactive_user2.active, inactive_user2.username
    );

    tuple_structs();
}

// Using Tuple Structs without Named Fields to Create Different Types
fn tuple_structs() {
    // Rust support struct that looks similar to tuples, called tuple structs.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
        Failed to destructure the Tuple Struct:

    let (r, g, b) = black;
    let (x, y, z) = origin;

    */

    println!("{}, {}, {}", origin.0, origin.1, origin.2);
    println!("{}, {}, {}", black.0, black.1, black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
