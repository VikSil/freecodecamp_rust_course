struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit struct don't have any fields
struct Unit;

fn main() {
    let mut user1 = User { // when a struct is instantiated, all values need to be provided
        active: true,
        username: String::from("username123"),
        email: String::from("address@domain.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newaddress@doman.com"); // the struct instance has to be mutable to do this

    // this is known as Struct Update Syntax
    let user2 = User {
        email: String::from("notuser1@domain.com"),
        ..user1 // instantiate using the same field values that another instance of the same Struct type has
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Success!");
}

fn make_user(email: String, username: String) -> User {
    // function makes an instance of a struct and returns it
    User {
        active: true,
        username, // short-hand syntax is valid
        email,
        sign_in_count: 1,
    }
}
