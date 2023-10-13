//function without a name, usually stored in a variable and can be used to pass a function into another function.

pub fn closer() {
    // let var_name = |parameters| -> return_type {BODY}

    let can_vote = |age: i32| age >= 18;
    println!("Can vote: {}", can_vote(15));
}

//can also access functions outside of it's body when its borrowing

pub fn borrowed_closer() {
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    //can change values if you mark a closure as mutable.

    let mut change_var = || samp1 += 4;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);
}

pub fn passed_closure() {
    //you can create functions inside of functions, but you usually wouldn't do this.

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }
    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("25 + 14 = {}", use_func(25, 14, prod));
}
