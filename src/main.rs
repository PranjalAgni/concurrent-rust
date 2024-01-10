use std::{thread, time::Duration};

fn main() {
    println!("Hello concurrent rust");
    let count_handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi {} number from spawned thread", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..=5 {
        println!("hi {} number from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    count_handle.join().unwrap();
}
