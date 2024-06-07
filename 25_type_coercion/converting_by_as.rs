// integer coercion is done by 'as' keyword
// 'as' coercions can be chained
// when typecasting an unsigned type T, T::MAX=1 is added or subtracted until the value fits into the new type
// e.g. largest possible unsigned 8-bit integer is 255 or 11111111, i.e. MAX u8 = 255 and MAX+1 = 256
// from there 1000 as u8 = 1000 - 256 = 744 - 256 = 488 - 256 = 232 <-- this value will be returned
// -1_i8 as u8 = MIN u8 -1 = 0_u8 - 1 = 255
// using unsafe typecasting methods can lead to undefined behaviours

#[allow(overflowing_literals)] // allows overflowing values to wrap around
fn main() {
    let decimal: f32 = 97.99_f32;

    let integer: u8 = decimal as u8; // typecasting will truncate (not round!) the fraction, value will be 97

    assert_eq!(97, integer);

    // let c1: char = decimal as char; // this would panic because decimal cannot be cast into a char
    let c1: char = decimal as u8 as char; // 97 is ascii for 'a'
    let c2: char = integer as char; // this is allowed because variable 'integer' is u8 already

    assert_eq!(integer + 1, 'b' as u8); // 'b' converted to u8 is 98

    println!("Success!");

    //------------------------------------------------

    assert_eq!(u8::MAX, 255);

    let v = 1000 as u8; // without overflow enabled this would panic, since the max u8 is 255

    println!("{}", v);

    println!("Success!");

    //------------------------------------------------

    assert_eq!(1000 as u16, 1000); // u16 can represent 1000, so this is ok
    assert_eq!(1000 as u8, 232); // for posititive number overflow cast is the same as modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255); // u8 is unsigned, and cannot be negative, hence it wraps around

    // since Rust v.1.45 the 'as' keyword performs a saturated cast when casting float to int. I.e. if the float exceeds the upper or lower bound, the retur value will equal the bound.
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    // saturated cast uncurs a small runtime cost and can be replaced with unsafe methods, but overflowing values will return unsound values

    unsafe {
        // to_int_unchecked in unsafe block will bypass checks in the compiler
        // 300 - u8::MAX = 200-256 = 44
        println!("300.0 is {}", (300.0_f32).to_int_unchecked::<u8>());

        // -100 + u8::MAX = -100 + 256 = 156
        println!("-100.0 as u8 us {}", (-100_f32).to_int_unchecked::<u8>());

        // 0, since that's the lower bound
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
