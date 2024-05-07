fn main() {
    let (mut x, y) = (1, 3);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 3);
    println!("Success!");
}
