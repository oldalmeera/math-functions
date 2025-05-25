// This is an example of using Rust's std::collections::HashMap to implement basic data manipulation.
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();

    // Adding elements to the map
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    map.insert(String::from("c"), 3);

    // Accessing and modifying values
    let val_a: i32 = *map.get(&String::from("a")).unwrap();
    println!("Value of a: {}", val_a); // Output will depend on the value of "b" in the HashMap

    let mut map_b = map.clone();

    // Updating values
    map_b.insert(String::from("d"), 4);

    // Iterating over the elements in the map
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
}
