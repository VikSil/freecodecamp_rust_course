// String is
// stored as a vector of bytes (Vec<u8>)
// always a valid UTF-8 sequence
// heap allocated
// growable
// not null terminated
// cannot be indexed into

// & str is
// a slice (&[u8])
// always points to a valid UTF-8 sequence
// can be used to view a String, because &[T] is a view into Vec<T>

fn main() {
    let mut s: String = String::from("Hello World");

    let slice0: &str = &s;
    let slice1: &str = s.as_str(); // works the same way as &s
    assert_eq!(slice0, "Hello World");
    assert_eq!(slice0, slice1);

    let slice2 = &s[..5]; // slice zero to four
    assert_eq!(slice2, "Hello");

    let slice3: &mut String = &mut s; // need a mutable reference to the String because it will be modified
    slice3.push('!'); // this is possible because immutable variable slice3 owns a mutable reference
    assert_eq!(slice3, "Hello World!");

    let mut slice4: String = s; // taking ownership of the String is ok, since s is not used past this point
    slice4.push('!'); // this is possible because the variable slice4 itself is mutable
    assert_eq!(slice4, "Hello World!!");

    println!("Success!");

    //------------------------------------------------

    let st: String = String::from("Hello World"); // String is allocated into Heap

    let slice: &str = &st; // this is just a view onto st, no new allocaiton to Heap are made

    let st: String = slice.to_string(); // .to_string() allocates result into Heap

    assert_eq!(st, "Hello World");

    println!("Success!");
}
