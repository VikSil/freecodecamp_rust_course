// TryFrom and TryInto are fallible conversions
// i.e. they return a Result type instead of a plain value
// i.e. they return converted value or an error if the conversion failed

use std::convert::TryInto;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct EvenNum(i32); // tuple struc with a single number

impl TryFrom<i32> for EvenNum {
    type Error = (); // associated type of type unittype

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            // if number is even
            Ok(EvenNum(value)) // return it wraped in the custom type
        } else {
            // otherwise
            Err(()) // return an error holding the associated unit type
        }
    }
}

fn main() {
    let n: i16 = 256;

    let n: u8 = match n.try_into() {
        // try to convert n of type i16 into another variable of the same name of type u8
        Ok(n) => n,
        Err(e) => {
            println!("this error occured while converting: {:?}", e.to_string());
            0 // if error happened, return zero
        }
    };

    assert_eq!(n, 0); //256 is out of bounds of u8, so conversion will fail and error handling block will return a zero

    println!("Success!");

    //------------------------------------------------

    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    let result: Result<EvenNum, ()> = (8i32).try_into();
    assert_eq!(result, Ok(EvenNum(8)));

    let result: Result<EvenNum, ()> = (5i32).try_into();
    assert_eq!(result, Err(()));

    println!("Success!");
}
