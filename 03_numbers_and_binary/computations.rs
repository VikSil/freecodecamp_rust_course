fn main() {
    //------------------------------------------------
    // Integer addition
    
    assert!(1u32 + 2 == 3); // the untyped numbers will be assumed to be of the same type

    //------------------------------------------------
    // Integer subtraction

    assert!(1i32 - 2 == -1);
    // assert!(1u8 - 2 == -1); // this will panic because compiler will try to assume that -2 and -1 is unsigned and also the sum of 1 and -2 is unsigned
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150); // by default all numbers will i32

    // assert!(9.6 / 3.2 == 3.0); // this will error if the type is not annotated because defaut f64 will come out with a tiny fraction after that 3.0
    assert!(9.6 as f32 / 3.2 == 3.0);

    assert!(24 % 5 == 4); // MOD operation (reminder)

    //------------------------------------------------
    // Short-circuiting boolean logic

    assert!(true && false == false); 
    assert!(true || false == true); 
    assert!(!true == false); 

    //------------------------------------------------
    // Bitwise operations

    println!("0011 AND 0101 is {:04b}", 0b001u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> is 0x {:x}", 0x80u32 >> 2);

    //------------------------------------------------

    println!("Success!");
}
