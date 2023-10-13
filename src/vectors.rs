fn vectors() {
    let vec1: Vec<String> = Vec::new();
    let mut vec2: Vec<String> = vec!["Baked", "Battered", "Fried", "Roasted", "Boiled"]
        .iter()
        .map(|s| format!("Cooking Method: {}", s))
        .collect();
    vec2.push("Cooking Method: Grilled".to_string());
    println!("1st type : {}", vec2[0]);

    let second: &String = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd cooking method found"),
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Total Cooking Methods : {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}

fn int_vectors() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![6, 7, 8, 9];
    vec2.push(10);
    println!("1st element: {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(2) {
        Some(third) => println!("2nd: {}", third),
        None => println!("No 3rd value :/"),
    }
    for i in &mut vec2 {
        *i *= 5;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vector Total Length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
}
