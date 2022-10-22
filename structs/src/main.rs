struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    // This struct is not mutable
    // To make it mutable you can add a mut keyword after let
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Creates another user with all the same info as user1 except for the email
    // User1 can no longer be used after decleration of user2 because of the username string being moved to user2
    // If user2 added username manually then user1 could be used again
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}

// Takes an email and username as parameters and returns a struct
// Active and sign_in_count are automatically assigned values
fn build_user(email: String, username: String) -> User {
    User {
        email, // Simply stating the variable name can only be done if the corresponding parameter name matches
        username,
        active: true,
        sign_in_count: 1,
    }
}
