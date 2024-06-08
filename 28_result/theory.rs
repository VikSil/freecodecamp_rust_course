// Result is an enum tye that represents an outcome of an operation that could potentially fail
// Result has two possible variants:
// Ok(T) - a value of the expected type T was found
// Err(e) - and error e was found

// Result Err can be returned for edge cases that would otherwise panic if not handled properly

// since Result is an enum, the variants can be checked against match pattern - this is akin to error handling

// the unwrap() method takes as input a value of type Result and extracts the value from Ok(T) in case of success, or panics if the Result is an Err(e)
// unwrap() is alternative to match, but since it panics in case of an Err(e) should only be used if it is 111% certain that an error will not occur

// returns f32 if successfull or &str if errror occured
fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        // return an error instead of panic
        return Err("Division by zero");
    }
    Ok(x / y) // return result wrapped in Ok because it has to be one of the variants of the Result
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg) => println!("Error: {}", msg),
    }
}
