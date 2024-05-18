fn main() {
    for n in 1..=100 {
        // equality sign includes the 100 in the range
        if n == 101 {
            // if this was 100, then panic would happen on the last iterration
            panic!("NEVER LET THIS RUN");
        }
    }
    println!("Success!");

    //------------------------------------------------

    let names: [String; 2] = [String::from("limining"), String::from("hanmeimei")];
    for name in &names {
        // each name is a copy of an element in names, ownership is retained to the array
        println!("{}", name); // technically it is *name, but println does dereferencing automatically
    }

    println!("{:?}", names); // :? prints via Debug trait. Arrays don't have a Display trait, they need to be printed via Debug trait

    let numbers: [i32; 3] = [1, 2, 3];
    for n in numbers {
        println!("{}", n);
    }

    println!("{:?}", numbers);

    //------------------------------------------------

    let a: [i32; 4] = [4, 3, 2, 1];

    for (index, value) in a.iter().enumerate() {
        // iter().enumerate takes a collection, e.g. array or tuple, and returns a tuple of index and value
        println!("The {}th element is {}", index + 1, value);
    }
}
