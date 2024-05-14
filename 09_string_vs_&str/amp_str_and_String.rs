fn main() {
    let s: &str = "Hello, World!"; // this is not a String, because it is hard coded, it is of type &str
    greetings(s.to_string()); // convert &str to String because that is what the funciton expects
    greetings(String::from(s)); // another way of converting &str to String

    //------------------------------------------------

    let s1: String = "Hello World!".to_string(); // &str of value "Hello World!" is converted to String by use of .to_String()
    let s2: &str = &s1; // use & to reference a String and pass that reference to variable of type &str
    let s3: &str = s1.as_str(); // .as_str() creates a reference to String, same as putting a & in front of it 
    // since s1 is passed on as a reference, there is no panic when doing so multiple times, because s1 still owns the "Hello World!" 

    println!("Success!");
    
}

fn greetings(s: String) { // function expects a String, not a &str
    println!("{}", s)
}