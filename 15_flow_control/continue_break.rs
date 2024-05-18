fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break; // exits the loop and n remains 66
        }
        n += 1;
    }

    assert_eq!(n, 66);
    println!("Success!");
    //------------------------------------------------

    let mut m = 0;
    for i in 0..=100 {
        if m != 66 {
            m += 1;
            continue; // will skip the remaining code in the current iteration and start the next iteration
        }
        break;
    }

    assert_eq!(m, 66);
    println!("Success!");
}
