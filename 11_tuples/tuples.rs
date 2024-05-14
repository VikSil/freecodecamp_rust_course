// Tuple is:
// a collection of elements that can be of different types
// compound type - i.e. type consisting of other types
// lives on the stack
// anotated as (T, T, T, ...)

fn main() {
    let _t0: (u8, i16) = (0, -1);

    let _t1: (u8, (u8, i16)) = (0, (0, -1)); // tuples can be nested

    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "Hello", String::from(", World!")); // can have differnt types

    println!("Success!");

    //------------------------------------------------

    let t2: (&str, &str, &str) = ("I", "am", "Sunface");
    assert_eq!(t2.1, "am"); // tuple indexing via dot notation, starts at zero

    println!("Success!");

    //------------------------------------------------

    let tuple_too_long = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("tuple too long: {:?}", tuple_too_long); // this tuple is too long to be printed and would cause a panic

    let tuple_not_too_long = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("tuple not too long: {:?}", tuple_not_too_long);

    //------------------------------------------------

    let tup: (i32, f64, &str) = (1, 6.4, "hello");

    let (x, z, y) = tup; // destructuring of the tuple

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");

    //------------------------------------------------

    let (a, b, c);

    (b, c, a) = (1, 2, 3);

    assert_eq!(a, 3);
    assert_eq!(b, 1);
    assert_eq!(c, 2);

    //------------------------------------------------

    let (x, y) = sum_multiply((2, 3)); // tuples can be used as function arguments and as return values

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) { // tuples can be used as function arguments
    (nums.0 + nums.1, nums.0 * nums.1) // tuples can be used as function return values
}
