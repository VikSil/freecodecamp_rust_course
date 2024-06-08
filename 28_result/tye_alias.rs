// since using std::result::Result<T, ParseIntError> everywhere is verbose and tedious, a type alias can be used instead

// aliases at module level can be particularly helpful because Errors found in a specific module often (don't?) have the same Err type, so single alias can be very sussinct when defining all associated Results. Example of this is standard library's io::Result.

use std::num::ParseIntError;

type Res<i32> = Result<i32, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str
        .parse::<i32>()
        .and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
}

fn print(result: Res<i32>) {
    match result {
        Ok(n) => println!("n in {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}
