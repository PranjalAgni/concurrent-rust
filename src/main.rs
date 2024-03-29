use std::sync::mpsc::{self, channel};
use std::sync::{Arc, Mutex};
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
            thread::sleep(Duration::from_secs(1));
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
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got {}", recieved);
    }
}

fn second_thread_example() {
    let valorant_handle = thread::spawn(|| {
        let character_name = String::from("Jett");
        println!("{}", character_name);
    });

    valorant_handle.join().unwrap()
}

fn api_of_mutexes() {
    let m = Mutex::new(5);
    {
        let mut m = m.lock().unwrap();
        *m = 7;
    }

    println!("mutex holds {:?}", m);
}

fn sharing_mutexes_between_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Result(sharing_mutexes_between_threads()): {}",
        *counter.lock().unwrap()
    );
}

fn main() {
    hello_concurrency();
    message_passing_with_concurrency();
    multiple_producers_by_cloning();
    second_thread_example();
    api_of_mutexes();
    sharing_mutexes_between_threads();
}
