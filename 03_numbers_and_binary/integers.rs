fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    // y = x; // this would fail due to differing types

    let mut z = 10; // by default this will be i32
    z = x; // hence this assignment will not fail
    println!("{z}"); // this will output the value of z because it is in curly braces (even though the syntax highlighter shows it as a literal string)

    //------------------------------------------------

    let v: u16 = 38_u8 as u16; // type conversion at assignment

    //------------------------------------------------

    let a = 5;
    //assert_eq!("u32".to_string(), type_of(&a)); // this would fail because default int type is i32 (signed int) not u32 (unsigned int)
    assert_eq!("i32".to_string(), type_of(&a));

    //------------------------------------------------

    assert_eq!(i8::MAX, 127); // maximum signed integer
    assert_eq!(u8::MAX, 255); // maximum unsigned integer

    //------------------------------------------------

    // let v1 = 251_u8 + 8; // the compiler will imply that v1 and 8 are also of type u8 and panic because 251+8 = 259, but maximum of u8 is 255
    let v1 = 251_u16 + 8; // change ot the next size of datatype that can handle the number

    // let v2 = i8::checked_add(251,8).unwrap(); // tihs will also panic because max for i8 is 127
    let v2 = i16::checked_add(251, 8).unwrap(); // change ot the next size of datatype that can handle the number

    println!("{}, {}", v1, v2);

    //------------------------------------------------

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
