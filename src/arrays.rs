fn integer_arrays_standard_loop() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // println!("1st : {}", arr_1[0]);
    // println!("Total Length : {}", arr_1.len());

    //loop through an array w index
    let mut loop_idx = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("Favourite Number under 10 : {}", arr_1[loop_idx]);
        loop_idx += 1;
    }
}

fn integer_arrays_while_loop() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // println!("1st : {}", arr_1[0]);
    // println!("Total Length : {}", arr_1.len());

    //loop through an array w index
    let mut loop_idx = 0;
    while loop_idx < arr_1.len() {
        println!("Array : {}", arr_1[loop_idx]);
        loop_idx += 1;
    }
}

fn integer_arrays_for_loop() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // println!("1st : {}", arr_1[0]);
    // println!("Total Length : {}", arr_1.len());

    //loop through an array w index
    let mut loop_idx = 0;
    for val in arr_1.iter() {
        println!("Values : {}", val);
    }
}
