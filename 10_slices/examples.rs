fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3]; // slices allow to borrow parts of collections without taking ownership of the entire thing

    assert_eq!(slice, &[2, 3]);

    println!("Success!");
}
