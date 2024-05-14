// arrays are:
// fixed length - known at the compile time
// containing elements of the same type
// stored in stack
// of type [Element type; Length]

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);
    println!("Success!");

    //------------------------------------------------

    let arr0 = [1, 2, 3]; // arrays don't have to be annotated, the compiler will infer that this is of type [i32;3]
    let arr1: [_; 3] = ['a', 'b', 'c']; // the underscore _ in type annotation also means - infer the type at compile time

    // std::mem::size_of_val - expects a pointer to a value, returns the number of bytes that a value takes up in memory
    assert!(std::mem::size_of_val(&arr1) == 12); // here 'a', 'b' and 'c' are unicode chars (not ascii chars that they would be if they were part of a String) and thus take 3 bytes each

    println!("Success!");

    //------------------------------------------------

    let list: [i32; 100] = [1; 100]; // all elements in an array can be initialised to the same value at once

    assert!(list[45] == 1); // all of the elements will be equal to 1
    assert!(list.len() == 100); // and there will be 100 elements

    println!("Success!");

    //------------------------------------------------

    // let _arr = [1,2, '3']; // this would panic because all elements in an array have to be of the same type
    let arr2: [i32; 3] = [1, 2, 3];
    let arr3: [char; 3] = ['1', '2', '3'];

    //------------------------------------------------

    let arr4: [char; 3] = ['a', 'b', 'c'];
    let el: char = arr4[1];

    assert!(el == 'b'); // array indexing starts at zero

    println!("Success!");

    //------------------------------------------------

    let words: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];

    let word0 = words.get(0).unwrap(); // using get(index) instead of [index] returns an Option<T> type and is considered a safer approach

    // let _word1 = &words[2]; // this would panic because index 2 is out of bounds

    println!("Success!");

    //------------------------------------------------
}

fn init_arr(n: i32) {
    // let arr = [1; n]; // this would panic because n is unknown at compile time. Arrays must be of known length
}
