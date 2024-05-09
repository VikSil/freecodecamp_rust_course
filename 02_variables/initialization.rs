fn main() {
    let x: i32 = 5;
    let y: i32; // uninitialized, but also not used, will only produce a Warning

    assert_eq!(x, 5); // x needs to be initialized by the time it is used here
    println!("Success!"); // will only execute, if the line above did not produce an error
}
