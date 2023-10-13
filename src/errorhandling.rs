fn error_handling() {
    //shows panic error example..
    let lil_arr = [1, 2];
    println!("{}", lil_arr[15]);
    // panic!("Awful Error");
}

//handles file IO operations
fn file_work() {
    let path = "line.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(File) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };
    write!(output, "some text to be made \nRandom").expect("Failed to write to file");

    let input = File::open(path).unwrap(); //unwrap ignores result and just gives output of file.
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File.create("rand.txt");
    let output2 = match output2 {
        Ok(File) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file due to: {:?}", error),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}
