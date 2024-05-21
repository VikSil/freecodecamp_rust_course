enum Direction {
    East,
    West,
    North,
    South,
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        }
        _ => println!("West"), // _ is used as a catch all pattern
    }

    //------------------------------------------------

    let boolean: bool = true;

    let binary: u8 = match boolean {
        // match used to assign value
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");

    //------------------------------------------------

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ]; // array of Message enum variants

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            // Message::Move has a struct-like syntax, not a tuple syntax, i.e. parameters are passed in curly braces
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("This variant has no data"), // _ is used to catch all patterns
    }
}
