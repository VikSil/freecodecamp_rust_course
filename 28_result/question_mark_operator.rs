// The ? operator is shorthand for propogating errors or unwrapping Ok() results
// The ? works the same as unwrap(), but it returns the error value instead of panicking
use std::num::ParseIntError;
use std::fs::File;
use std::io::{ self, Read };

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    // tries to parse the string into an i32, but coudl fail, since it is not all strings are numbers
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => {
            return Err(e);
        }
    };
    println!("{}", number);

    // Alternativelly
    let number = number_str.parse::<i32>()?; // replaces the entire match pattern
    println!("{}", number);

    //------------------------------------------------

    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");

    //------------------------------------------------

    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");

    Ok(()) // need to return smthn because that's what the function definition says
}

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: i32 = n1_str.parse::<i32>()?; // the ? inherently unwraps the parse result
    let n2: i32 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

fn read_file1() -> Result<String, io::Error> {
    //function will result in text form file or IO Error
    let f: Result<File, io::Error> = File::open("hello.txt"); // try to open the file
    let mut f = match f {
        // get file or missing file error
        Ok(file) => file, // if ok unwrap the file and proceed
        Err(e) => {
            return Err(e);
        } // if error, throw it upwards
    };

    let mut s: String = String::new(); // create a string that will hold the file text
    match f.read_to_string(&mut s) {
        // read file contents into String via mutable reference, as defined in std
        Ok(_) => Ok(s), // general matched or Ok containing something
        Err(e) => Err(e),
    }
}

fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    // File:: open returns a Result that is then unwrapped by ?
    // and chained into read_to_string pointing to the String variable
    // and Result of read_to_String is then also unwrapped by ?
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
