struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello {id:i32},
}

fn main() {

    let a:i32 = 5;
    let b:i32 = 7;
    let c:i32 = 11;
    match_number(a);
    match_number(b);
    match_number(c);

    //------------------------------------------------

    let p:Point = Point {x: 4, y: 10};

    match p {
        // struct matching syntax Structname {field1:value1, field2:value2}
        Point {x,y:0} => println!("On the x axis at {}",x), // when written in shorthand notation, i.e. x  is the same as x:x matching is also a destructuring assignment, i.e. the second unspoken x is what is being printed out
        Point {x:0..=5, y: y@ (10|20|30)} => println!("On the y axis at {}", y), // @ operator creates a variable and hence y can be printed out
        Point {x,y} => println!("On none of the axis ({},{})", x, y),
    }


    //------------------------------------------------

    let msg:Message = Message:: Hello {id:5};

    match msg { // different destructuring syntaxes
        Message::Hello{id:id@ 3..=7,} =>println!("Found an id in range [3,7]: {}", id),
        Message::Hello{id: newid@ (10 | 11| 12)} => {println!("Found an id in range [10,12]: {}", newid)},    
        Message::Hello{id} => println!("Found some other id: {}", id),
    }

    //------------------------------------------------

    // match guard is an additional if condition specified after the pattern in a match arm
    // match guard must also be true along with the pattern matching for that arm to be true

    let num: Option<i32> = Some(4);
    let split:i32 = 5;
    match num {
        Some(x) if x < split =>assert!(x < split), // 'if x < split' is the match guard
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");

    //------------------------------------------------

    let numbers = (2,4,8,16,32,64,128,256,1024,2048);

    match numbers {
        (first, .., last) => { // .. ignores the remaining parts of the value that are not explicitly assigned
            assert_eq!(first,2);
            assert_eq!(last,2048);
        }
    }

    println!("Success!");

    //------------------------------------------------
    let mut v: String = String::from("Hello,");
    let r: &mut String = &mut v;

    match r {
       value =>value.push_str(" World!") // since r is the thing being matched it will automatically be destructured into v with this syntax
    }

    println!("{}",v);
    println!("Success!");

    //------------------------------------------------

}

fn match_number(n: i32) {
    match n {
        1 => println!("match one (but as integer)"),
        2 | 5 => println!("match 2 OR 5"),
        6..=10 => {
            println!("match 6 TO 10");
        },
        _ => {
            println!("match anything");
        },
    }
}

