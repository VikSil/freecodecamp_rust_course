fn main() {
    let s1 = gives_ownership(); // function gives ownership of its return value to s1

    let s2 = String::from("hello"); // s2 comes into scope and owns the pointer to "hello"

    let s3 = takes_and_gives_back(s2); // s2 passes its ownership of pointer to "hello" to the function, and function passes the ownership of its return value to s3
    // it just so happens that the functions return value is the same pointer it took over from s2 variable
    // s3 now owns the pointer to the same "hello" that s2 used to own
    // s2 ceases to exist once its value has been moved into the function

    println!("Success!");
}// s1 and s3 cease to exist

fn gives_ownership() -> String {
    // will return ownership to a pointer to a sting
    let some_string = String::from("Here is your value"); // some_string comes into scope and gains ownership of pointer to a string

    some_string // ownership that some_string had is returned to the caller and some_string ceases to exist
}

// function receives and will return ownership of a pointer to a string
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope and takes ownership of the pointer that gets passed into the function

    a_string // ownership that a_string had is returned to the caller and a_string ceases to exist
}
