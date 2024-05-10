fn main() {
    let x: u32 = 5u32;

    // the assignment to y is a statement, since it does not return a value
    let y = {
        // the entire content of the curly braces is an expression, since it returns a value
        let x_squared = x * x;
        let c_cubed = x_squared * x;

        // value of this expression will be assigned to y
        c_cubed + x_squared + x // there is no semicolon, hence this is a return statement
    }; // y will be of type u32, since it is derived from x which is explicitly u32

    let z: () = {
        2 * x; // the semicolon suppresses this expression and '()' will be assigned to z
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    //------------------------------------------------

    let a = {
        // the type of a will be () and if it was annotated as anything else, compiler would panic
        let mut b: i32 = 1;
        b += 2; // b will become three, but since this is an assignment (statement), it will not be returned
    };

    assert_eq!(a, ());
    println!("Success!");

    //------------------------------------------------

    let c: i32 = {
        let mut d: i32 = 1;
        d += 2;
        d // this will be returned and assigned to c
    };

    assert_eq!(c, 3);
    println!("Success!");

    //------------------------------------------------

    // let e = (let f = 3); // this is invalid syntax

    let e: i32 = {
        let f: i32 = 3;
        f
    };

    assert!(e == 3);

    println!("Success!");

    //------------------------------------------------

    let s: i32 = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");

    fn sum(x: i32, y: i32) -> i32 {
        x + y // if there was semicolon here, then nothing would be returned and the result of sum would be () regardless of how the function is annotated
    }
}
