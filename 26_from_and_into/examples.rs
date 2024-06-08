fn main() {
    let i1:i32 = false.into();
    let i2:i32 = i32::from(false);
    assert_eq!(i1,i2);
    assert_eq!(i1,0);

    let i3: u32 = 'a'.into(); // out of the box chars only convert to u integer types, not i interger types because a char cannot be negative

    // let s: String = 'a' as String; // as keyword does not work for chars to String
    let s: String = String::from('a');
    let s1: String = 'a'.into();

    //------------------------------------------------

    println!("Success!");
}