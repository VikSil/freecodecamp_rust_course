use std::num::ParseIntError;

fn add_two_via_map(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|n| n + 2)
}

fn add_two_via_and_then(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|n| Ok(n + 2)) // unlike map has to wrapped in OK
}

fn main() {
    assert_eq!(add_two_via_map("4").unwrap(), 6);
    assert_eq!(add_two_via_and_then("4").unwrap(), 6);

    println!("Success!");

    //------------------------------------------------

    let twenty: Result<i32, ParseIntError> = multiply_rewritten("10", "2");
    print(twenty); // this will output the unwrapped Ok value

    let tt: Result<i32, ParseIntError> = multiply_original("t", "2");
    print(tt); // this will outut the error message

    println!("Success!");
}

fn multiply_original(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => {
            match n2_str.parse::<i32>() {
                Ok(n2) => Ok(n1 * n2),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// does the same as multply_original, but in a less verbose way
fn multiply_rewritten(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str.parse::<i32>().and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
}

fn print(result: Result<i32, ParseIntError>) {
    // since both multily functions return a Result, the variants need to be unwrapped before printing
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
