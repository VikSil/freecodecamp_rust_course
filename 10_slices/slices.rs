// slices are similar to arrays, but their length is not known at compile time
// hence slices are always refered to, not used directly

fn main() {
    let arr: [i32; 3] = [1, 2, 3]; // array type is direct and of known length
    let s1: &[i32] = &arr[0..2]; // slice type is referenced and of unknown length

    let s2: &str = "Hello, World!"; // string literals are always &str, i.e. it is a reference to where in the program's binary itself it is hardcoded

    println!("Success!");

    //------------------------------------------------

    let arr1: [char; 3] = ['大', '木', '土']; // each of these unicode chars are 4 bytes
    let slice: &[char] = &arr1[..2]; // zero can be ommited if slice starts at zero

    assert!(std::mem::size_of_val(&slice) == 16); // slice is 16 bytes because it is a pointer. The size of a slice has nothing to do with the length of it. The slice is 8 bytes for the pointer itself and 8 bytes for th elength of the value

    println!("Success!");

    //------------------------------------------------
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let slice2: &[i32] = &arr2[1..4];
    assert_eq!(slice2, &[2, 3, 4]);

    println!("Success!");
}
