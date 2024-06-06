// hashmaps in Rust
// store key-value pairs
// where each key is hashed to a unique index in the underlying array
// accessing of elements/entries are at best constant time O(1) at worst linear time O(n)
// are allocate into Heap
// can dynamically change size
// used for lookups
// by default uses SipHash 1-3 hashing algorythm that provides resistance against HashDoS attacks
// SipHash 1-3 works vell for medium sized keys, but other hashing algorythms will outperform it for small keys (integers)
// or large keys (long strings)
//

use std::collections::HashMap; // other standard types are in the prelude (autoloaded), but HashMap needs to be loaded

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniela", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58); // all keys and values have to be of the same type, since it is a vector pointing to an array

    let score: Option<&i32> = scores.get("Sunface"); // .get() returns Option<&V>, this is safer than indexing
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniela") {
        let score: i32 = scores["Daniela"]; // indexing returns V
        assert_eq!(score, 95);
        scores.remove("Daniela");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score); // will print in random order
    }

    //------------------------------------------------

    let teams: [(&str, i32); 3] = [ // array of three tuples of string literal and i32 integer
        ("Chinese Team", 100),
        ("American Team", 10),
        ("French Team", 50),
    ];

    let mut teams_map1: HashMap<&str, i32> = HashMap::new();
    for team in &teams {// for each element in the array, insert key and value into the hashmap
        teams_map1.insert(team.0, team.1);
    }

    let teams_map2: HashMap<&str, i32> = HashMap::from(teams);
    let teams_map3: HashMap<&str, i32> = teams
        .into_iter()
        .map(|e| *e)
        .collect();

    assert_eq!(teams_map1, teams_map2);
    assert_eq!(teams_map3, teams_map2);

    println!("Success!");

    //------------------------------------------------    

    let mut player_stats = HashMap::new(); // type of this will be inferred as HashMap<&str, u8>

    player_stats.entry("health").or_insert(100); // access or insert key if it doesn't already exist

    assert_eq!(player_stats["health"],100);

    player_stats.entry("health").or_insert_with(random_stat_buff); // insert from a function if key doesn't already exist
    assert_eq!(player_stats["health"], 100); // the value will be 100 still because it already existed when .or_insert_with() was called

    // health will be mutable because the HashMap is mutable to begin with
    let health: &mut u8 = player_stats.entry("health").or_insert(50); // health will always be found because if it is not in the HashMap, it will be inserted and returned to the variable
    assert_eq!(health, &100);
    *health -= 50; // dereferencing and decreasing the value in HashMap
    assert_eq!(*health, 50); // comparison to the dereferenced value
    assert_eq!(player_stats["health"],50); // direct comparison of the HashMap

    println!("Success!");

}

fn random_stat_buff() -> u8 {
    42
}
