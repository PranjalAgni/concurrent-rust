use std::sync::mpsc::{self, channel};
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

fn multiple_producers_by_cloning() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    });

    for recieved in rx {
        println!("Got {}", recieved);
    }
}

fn main() {
    hello_concurrency();
    message_passing_with_concurrency();
    multiple_producers_by_cloning();
}
