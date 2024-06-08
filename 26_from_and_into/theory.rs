// from trait allows a type to define how to create itself from another type
// into trait allows another type to be converted into a type
// 'from' and 'into' traits:
// are part of standard library
// are used for type conversions between different types without requiring explicit casts
// can be implemented for custom types
// into is reciprocal of the from trait. I.e. implementing 'from' for a type will double as 'into' implementation for the other type
// e.g. impl From<T> for U allows both
// u:U = U::from(T) and
// let u:U = T.into()

#[derive(Debug)]
struct Number { // this is a a custom struc type
    value: i32, // with one field of type i32
}

// From trait implementation for converting i32 to the custom type
impl From<i32> for Number {
    fn from(n: i32) -> Number {
        // take the i32
        Number {
            value: n, // and make it a value for the value field in Number type
        }
    }
}

fn main() {
    let num: Number = Number::from(30); // call From on Number constructor
    assert_eq!(num.value, 30);

    let num: Number = (30_i32).into(); // call into on the convertable type to turn a value into custom type that is expected
    assert_eq!(num.value, 30);

    println!("Success!");

    //------------------------------------------------

    let my_str = "hello";

    // all of the below are possible because String implements From<&str>
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    let string3: String = my_str.into(); // type annotation is required here for the compiler to know what to turn the &str into

    println!("Success!");
}
