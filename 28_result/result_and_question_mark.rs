use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: Result<i32, ParseIntError> = n1_str.parse::<i32>();
    let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result: Result<i32, ParseIntError> = multiply("10", "2");
    assert_eq!(result, Ok(20));

    // let result: Result<i32, ParseIntError> = multiply("t", "2"); // this would return an error because "t" cannot be parsed into an integer
    let result: Result<i32, ParseIntError> = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}
