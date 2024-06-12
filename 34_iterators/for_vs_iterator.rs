fn main() {
    let v = vec![1, 2, 3];
    for x in &v {
        // by default for will apply the into_iter to a collention and change it into an interator
        println!("{}", x);
    }

    for x in v.into_iter() {
        // works the same way as the above for loop
        println!("{}", x);
    }

    //------------------------------------------------

    let arr = [0; 3]; // an array of three zeroes
    for i in 0..arr.len() {
        // for loop over non inclusiverange
        println!("{}", arr[i]);
    }
    // loop above refactored with an iterrator
    for i in arr.into_iter() {
        println!("{}", i);
    }
    // refactored even more
    for i in arr {
        println!("{}", i);
    }

    //------------------------------------------------

    // one of the easiest ways to create an iterator is to use the range notion a..b

    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }
    assert_eq!(v.len(), 100);
}
