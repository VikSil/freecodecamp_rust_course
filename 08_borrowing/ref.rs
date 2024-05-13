// ref can be used to access references to a value, similar to &

fn main() {
    let c: char = '牛';

    let r1: &char = &c;
    let ref r2 = c; // this results in r2 holding a pointer to c, same exact as r1 in the line above

    assert_eq!(*r1, *r2);

    // check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r) // this works the same as println!, except it returns the output as a String rather than putting it in the console
}
