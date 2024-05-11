fn main() {
    let s = String::from("hello"); // s comes into scope
                                   // the value of s is a string and strings are passed around via pointers

    takes_ownership(s); // the pointer to "hello" is moved into the function, i.e. function takes ownership of the value that was previously assigned to s
                        // since ownership was taken from s, the variable s no longer exists going forward

    let x = 5;  // x comes into scope 
                // the value of x is of type i32, it goes directly into stack
 
   makes_copy(x); // the value of x gets copied into the function, but x maintains ownership of the original value
                  // since x did not loose ownership, it continues to exist

}
// x goes out of scope and no longer exists. 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called automatically to free the memory it was pointing at

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}// some integer goes out of scope. Since it is not a pointer there is no need to call 'drop' to free memory
