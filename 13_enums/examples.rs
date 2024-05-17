enum ipAddr { // this enum holds two variants (not fields)
    V4(String),
    V6(String),
}

fn main() {
    let home = ipAddr::V4(String::from("127.0.0.1")); // enum get instantiated by one of the variants, not all

    let loopback = ipAddr::V6(String::from("::1"));

    println!("Success!");
}
