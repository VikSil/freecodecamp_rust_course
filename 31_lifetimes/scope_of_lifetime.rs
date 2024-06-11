// below variable i has the longest lifetime because its scope encloses both variables borrow1 and borrow2
//the duration of borrow1 compared to borrow2 is irrelevant because they don't interact

fn main() {
    let i: i32 = 3; // i lifetime starts

    {
        let borrow1 = &i; // borrow1 lifetime starts

        println!("borrow1: {}", borrow1);
    } // borrow1 lifetime ends
    {
        let borrow2 = &i; // borrow2 lifetime starts
        println!("borrow2: {}", borrow2);
    } // borrow2 lifetime ends
} // i lifettime ends
