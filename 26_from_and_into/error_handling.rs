// when handling errors it is often useful to implement From trait for custom error types.

use std::fs;
use std::io;
use std::num;

enum CliError { // this enum holds error types form the standard library
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

// implementation to convert standard error to custom enum error
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IoError(err) // creates an instance of the correct enum variant
    }
}

// implementation to convert standard error to custom enum error
impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> Self {
        // Either Self or the type name is allowed
        CliError::ParseError(err) // creates an instance of the correct enum variant
    }
}

// function resutts in i32 if successful or CliError if not
fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // open file and get the contents
    // ? is for exception handling and automatically converts io::Error to CliError
    let contents: String = fs::read_to_string(&file_name)?; // fs module is used for file manipilation

    // parse the contents of the file
    // if parsing fails then num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;

    Ok(num)
}

fn main() {
    println!("Success!");
}
