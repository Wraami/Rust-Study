use std::collections::HashMap;

pub fn hashmaps() {
    let mut musicians = HashMap::new();
    musicians.insert("Grouper", "Ambient Artist");
    musicians.insert("Rina Sawayama", "Pop Artist");
    musicians.insert("Charli XCX", "Pop Artist");
    musicians.insert("Alex G", "Indie Artist");

    for (k, v) in musicians.iter() {
        println!("{} = {}", k, v);
    }

    println!("Total Artists in database: {}", musicians.len());
    //should be very safe about what we're doing in rust.

    //checking for specific key in a hashmap.
    if musicians.contains_key(&"Grouper") {
        let the_grouper = musicians.get(&"Grouper");
        match the_grouper {
            Some(x) => println!("wash away to the ocean"),
            None => println!("And my head hurts"),
        }
    }
}
