#[derive(Debug)] //the #[deriv(Debug)] derived trait makes a struct printable
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale: u32 = 2;
    let rectangle1: Rectangle = Rectangle {
        width: dbg!(30 * scale), // simultaneously prints debug info to stderr and assigns the value to width
        height: 50,
    };

    dbg!(&rectangle1); // prints debug info to stderr

    println!("{:?}", rectangle1); // prints debug info to stdout
}

// stderr - where the errors (and debug) are sent to
// stdout - where normal output is sent to
// these are usually either both console, or different log files
