pub fn new_variable_assignments() {
    const ONE_MILLI: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "21";
    // let mut age:u32 = 21;
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MILLI);
}

pub fn max_min_variables() {
    println!("max u32: {}", u32::MAX);
    println!("max u64: {}", u64::MAX);
    println!("max usize: {}", usize::MAX);
    println!("max u128: {}", u128::MAX);

    println!("max f32: {}", f32::MAX);
    println!("max f64: {}", f64::MAX);

    let is_true = true;
    // let my_grad= 'D';
    //     println!(my_grad);
}
