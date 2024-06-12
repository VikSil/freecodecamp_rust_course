fn main() {
    let arr: Vec<i32> = vec![0;10];
    // iter because default for loop would call into_iter and consume the arr and would not allow to print it later
    for i in arr.iter() {
        println!("{}", i); // having immutable reference to read is sufficient for printing
    }

    println!("{:?}", arr);

    //------------------------------------------------

    let mut names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        // .iter_mut because the for loop mutates the values and they need to be available to print after the loop
        *name = match name {
            //dereferencing and referenceing, since iter_mut() loop does not take ownership
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        };
    }

    println!("names: {:?}", names);

    //------------------------------------------------

    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.next() {
        *v = 0; // dereference, since iter_mut() gives mutable reference
    }

    assert_eq!(values, vec![0, 2, 3]);
    println!("{:?}", values);
}
