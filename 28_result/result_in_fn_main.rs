// typically th emain function will look like this:

// fn main(){
//      println!("Hello World!");
// }

// however main can also return a Result
// i.e. if an error occurs in main, it will return an error code and print debug representation of the error (via Debug trait)

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str: &str = "10";
    // if conversion fails, this will return the error variant and exit the main
    let number: i32 = number_str.parse::<i32>()?; // if main did not return a Result, it would not be possible to use ? inside main like this, since ? can return an error

    println!("{}", number);

    Ok(()) // the Result Ok with a unit type needs to be returned, if the function did not exit with an error earlier
}
