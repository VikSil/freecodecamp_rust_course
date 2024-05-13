fn main() {
    let s1 = String::from("hello"); // s1 is stored in stac and it is a pointer to a String in heap

    let len = calculate_length(&s1); // the function takes ownership of the pointer to s1, NOT the pointer value that s1 is an owner of

    println!("The length of '{}' is {}", s1, len); // s1 is still valid here because it never lost ownership of its value
}

fn calculate_length(s: &String) -> usize { // s value is of type pointer to String, not type String, like s1 is
    s.len()  // len is of type i32, hence it will be copied, not passed to a new owner
    // both s and s1 live on the stack
}