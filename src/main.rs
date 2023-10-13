#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;

mod arrays;
mod closers;
mod concurrency;
mod enums;
mod functions;
mod hashmaps;
mod helpers;
mod math;
mod pizzeria;
use crate::pizzeria::order_food;
mod smartpointers;
mod strings;
mod structs;
mod variables;
mod vectors;

// mod random_values;
// mod conditionals;

pub fn main() {
    concurrency::concurrency_use();
    // closers::borrowed_closer();
    // order_food();
    // let mut str1 = String::from("Holy Moly :0");
    // // let str2 = str1.clone();
    // // // print_str(str2);
    // // let str3 = print_return_str(str1);
    // // println!("{}", str3);
    // change_string(&mut str1); // Call the function with a mutable reference to str1
    // change_string(str2);
    // println!("12 + 17 = {}", get_sum_generic(12, 17)); //integers
    // println!("12.7 + 17.4 = {}", get_sum_generic(12.7, 17.4)); //floats

    // let (val_1, val_2) = get_2(7);
    // println!("Numbers : {} {}", val_1, val_2);
    // let num_vec = vec![8, 12, 16, 20];
    // println!("Sum of vector list = {}", sum_list(&num_vec));
    // println!("{}", get_multi_2(20, 40));
    // get_multi(40, 20);
    // hashmaps::hashmaps();
    // structs::generic_structs();
    // structs::structs();
    // math::math();
    // variables::new_variable_assignments();
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't receive input");
    // println!("Hello {}! {}", name.trim_end(), greeting);
    // println!("Hello," mut);
}

fn random_values() {
    let random_num = rand::thread_rng().gen_range(1..101);

    println!("Random : {}", random_num);
}

fn conditional_expression() {
    let age = 8;

    if (age < 18) && (age > 1) {
        println!("your birthday sucks kid")
    } else if (age >= 60) {
        println!("pension time!");
    } else {
        println!("amazing birthday job.");
    }
}

fn ternary_operators() {
    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };

    println!("Can vote: {}", can_vote);
}

fn match_error_handling() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("baby birthday"),
        21 | 50 => println!("cool alcholic birthday"),
        65..=i32::MAX => println!("pension birthday"),
        _ => println!("not a real birthday"),
    }
}

fn voter_check() {
    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("ineligible voter"),
        Ordering::Greater => println!("eligible voter"),
        Ordering::Equal => println!("Just gained voter rights!"),
    }
}

fn tuples() {
    let my_tuple: (u8, String, f64) = (47, "Trish".to_string(), 50_000.0);

    println!("Name : {}", my_tuple.1);
    println!("Age : {}", my_tuple.0);
    println!("Bank balance : {}", my_tuple.2);

    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}

fn casting() {
    //two types of casting.

    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    //can cast using as

    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}

//we can't use addition operator on generics in this form.
//this is an add trait, and specifies the use for different types.
// fn get_sum_generic<T>(x: T, y: T) -> T {
//     return x + y;
// }

fn get_sum_generic<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
