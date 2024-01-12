use std::sync::mpsc;
use std::{thread, time::Duration};

fn hello_concurrency() {
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

fn message_passing_with_concurrency() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let name = String::from("Pranjal");
        tx.send(name).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn main() {
    hello_concurrency();
    message_passing_with_concurrency();
}
