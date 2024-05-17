enum Number {
    Zero, // if no value is provided, then first value is implicitly 0
    One, // sencond is implicitly 1
    Two, //etc.
}

enum Number1 {
    Zero = 0, // explicit enumeration
    One,
    Two,
}

/*
enum Number2 {
    Zero = 0.0, // floating point enumaration is not allowed
    One = 1.0,
    Two = 2.0,

}
*/

enum Number2 {
    Zero = 2,
    One, // this will implicity assume value 3
    Two, // this will implicitly assume value 4
    Three = 6, // this is ok, value 5 is skipped
}

#[derive(Debug)] // trait needed for printing
enum Message { // each enum variant can hold fields of different datatypes, i.e. a variant can be a struct or a tuple
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    assert_eq!(Number::One as u8, Number1::One as u8); // 'as' keyword is used to convert enum variant to an integer, without convertion it will panic
    assert_eq!(Number1::Two as u8, Number2::Zero as u8);

    println!("{}, {}, {}", Number::Zero as u8, Number1::One as u8, Number2::Two as u8);

    println!("Success!");

    //------------------------------------------------

    // both of the below are of the same type, just different variants
    let msg1: Message = Message::Move { y: 1, x: 2 }; // instantiating with x = 1, y = 2, the field names can come in any order, but they must be given, simply writing {1,2} will panic
    let msg2: Message = Message::Write(String::from("Hello, World!")); // instantiating with "Hello World!"

    println!("Success!");

    //------------------------------------------------

    let msg: Message = Message::Move { x: 1, y: 1 };

    if let Message::Move { x: a, y: b } = msg {
        // destructing enum of type  Message::Move into variables a and b
        assert_eq!(a, b);
    } else {
        panic!("Don't run this!");
    }

    println!("Success!");

    //------------------------------------------------

    let msgs: [Message; 3] = [
        // [Message; 3] is type annotation of the array - array of three elements of type Message
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}
