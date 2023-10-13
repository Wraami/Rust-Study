fn strings_mut_and_push() {
    let mut st1 = String::new();
    st1.push('D');
    st1.push_str(" AMN!");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("D", "Another D");
    println!("{}", st2);
}

fn strings_random_char() {
    let st3 = String::from("x r t u z i r x p a p a");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup(); //removes duplicates
    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Rando string"; //string literal.
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String Length : {}", st6.len());
    //delete values if mutable
    st5.clear();

    // combine string
    let st6: String = String::from("Some Yummy ");
    let st7: String = String::from("drywall & Abestos");
    let st8 = st6 + &st7;
    println!("{}", st8);
    for char in st8.bytes() {
        println!("{}", char);
    }
}

fn deallocation_string() {
    let str1 = String::from("HolyMoly");
    let str2 = str1.clone();

    println!("{}", str1);
}

fn print_str(x: String) {
    println!("string named: {}", x);
}

fn print_return_str(x: String) -> String {
    println!("string named : {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" Kitty!!!!");
    println!("Message: {}", name);
}
