// The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a
// hashing function, which determines how it places these keys and values into memory.

// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved and the hash map will be the owner of those values.
//If we insert references to values into the hash map, the values won’t be moved into the hash map.
// The values that the references point to must be valid for at least as long as the hash map is valid.

pub fn run_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in scores {
        println!("{key}: {value}");
    }

    // Adding a Key and Value Only If a Key Isn’t Present

    // call to entry will not change the hash map because the Blue team already has the value 10.
    scores.entry(String::from("Blue")).or_insert(50);

    // Updating a Value Based on the Old Value
}
