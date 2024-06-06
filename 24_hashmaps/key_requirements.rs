// any type that implements Eq and Hash traits can be a key in HashMap
// out of the box the types are bool, int, unit, String and &str
// f32 and f64 do not implement Hash because floating-point precision errors would make using them as HashMap keys very error-prone
// all collection classes implement Eq and Hash if the types they contain implement those traits
// i.e. Vec<T> implements Hash if T implements Hash

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let vikings: HashMap<Viking, i32> = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
