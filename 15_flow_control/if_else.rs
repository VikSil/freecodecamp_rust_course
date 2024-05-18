fn main() {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
    //------------------------------------------------

    let big_n: i32 = // if-else assignment
        if n < 10 && n > -10 {
            println!("and is a small number, increase ten-fold");
            10 * n // no semicolon - this line will be returned
        } else {
            println!("and is a big number, half it");
            n / (2.0 as i32) // typecasting because n is i32
        }; // assigments are statements and must end with a semicolon

    println!("{} -> {}", n, big_n);
}
