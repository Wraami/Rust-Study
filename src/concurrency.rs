use std::thread;
use std::time::Duration;

pub fn concurrency_use() {
    let thread1 = thread::spawn(|| {
        for i in 1..26 {
            println!("Spawned Thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..21 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap();
}
