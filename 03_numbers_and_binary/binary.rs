fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //decimal + hex + octal + binary
    // 1_024 = 1 024 , i.e. 1024 with a delimiter
    // 0xff = 255
    // 0o77 = 63
    //0b1111_1111 = 255

    assert!(v == 1597);

    println!("Success!");
}
