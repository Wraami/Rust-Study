//Functions!
fn say_hi_hi() {
    println!("Hi hi hi!");
}

//passing arguments to a function
fn get_multi(x: i32, y: i32) {
    println!("{} * {} = {}", x, y, x * y);
}

//since we are returning a value, define the data type thats being returned.
fn get_multi_2(x: i32, y: i32) -> i32 {
    x * y
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}
