fn main() {
    let b = false;

    let _v = match b {
        true => 1,
        // diverging functions can be used in match expression to raise errors
        false => {
            println!("Success!");
            panic!("There is no value for flase, let's panic");
        }
    };

    println!("Exercise  failed if this line is being printed out");
}
