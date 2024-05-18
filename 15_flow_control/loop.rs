fn main() {
    let mut count: u32 = 0u32;

    println!("Let's count until infinity!");

    loop {
        // this is an infinite loop that is controlled by break and continue
        count += 1;

        if count == 3 {
            println!("three");
            continue; // skip the rest of this iteration
        }
        println!("{}", count);

        if count == 5 {
            println!("Actually, that's enough");
            break; // will quit the loop
        }
    }

    assert_eq!(count, 5);
    println!("Success!");

    //------------------------------------------------

    let mut counter = 0;

    let result: i32 = loop {
        // when the loop exits, it will return a value into result variable
        counter += 1;

        if counter == 10 {
            break counter * 2; // break out of the loop and return counter *2
        } //Semicolon because the entire loop is an assignment. This semicolon can also be placed at the end of the previous line instead
    };

    assert_eq!(result, 20);
    println!("Success!");

    //------------------------------------------------

    // it is possible to continue and break outer loop from a nested loop
    let mut c: i32 = 0;
    'outer: loop {
        'inner1: loop {
            if c >= 20 {
                break 'inner1; // has the same effect as simply 'break' - only escapes the most immediate loop
            }
            c += 2;
        }

        c += 6;

        'inner2: loop {
            if c >= 30 {
                break 'outer; // escapes the inner2 and outer loop
            }

            continue 'outer; // skips to the next iteration of the outer loop
        }
    }

    assert!(c == 32);
    println!("Success!");
}
