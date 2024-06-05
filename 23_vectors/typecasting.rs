fn main() {
    // Array --> Vec
    // impl From<[T: N]> for Vec

    let arr: [i32; 3] = [1, 2, 3];
    let v1: Vec<i32> = Vec::from(arr); // calls vecotr constuctor and initiates it with array as a template
    let v2: Vec<i32> = arr.into(); // calls a method on arr and converts arr object into a vector

    assert_eq!(v1, v2);

    // String --> Vec
    // impl From<String> for Vec

    let s: String = "hello".to_string();
    let v1: Vec<u8> = s.into(); // converts String into a vector

    let s: String = "hello".to_string();
    let v2: Vec<u8> = s.into_bytes(); // works the same as .into() for String type
    assert_eq!(v1, v2);

    // &str --> Vec
    // impl<'_> From<&'_ str> for Vec

    let s: &str = "hello";
    let v3: Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);

    // iterators can be collected into vectors

    // [0; 10] is ten elements of value zero
    let v4: Vec<i32> = [0; 10]
        .into_iter()
        .map(|e| *e)
        .collect(); // map dereferences the values iterator returns
    // https://stackoverflow.com/questions/49727495/why-do-i-get-the-error-fromiteratorinteger-is-not-implemented-for-veci32
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
}
