enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // all of the possible values have to be matched, therwise the compilier will panic
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin: Coin = Coin::Nickel;
    let amount = value_in_cents(coin);
    println!("{}", amount);
}
