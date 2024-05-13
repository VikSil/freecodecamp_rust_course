fn main() {
    let mut s = String::from("Hello"); // the variable has to be mutable

    println!("s before change is {}", s);
    change(&mut s); // and the pointer to the variable must also be mutable

    println!("s after change is {}", s);
}

fn change(some_string: &mut String) { // the input must also be annotated as a mutable
    some_string.push_str(", World!"); //all through this s and not some_string is still the owner of the value
}