# Concurrency in Rust

- Using threads to run concurrent code

  The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread. There are crates that implement other models of threading that make different tradeoffs to the 1:1 model.

* Basic:: spawning a new thread with spawn

  ```rs
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
  }
  ```

* Waiting for the spawned thread to complete

  Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. Blocking a thread means that thread is prevented from performing work or exiting. Because we’ve put the call to join after the main thread’s for loop

* We can clone transmitter and send messages to single reciever. For example:

```rs
let (tx, rx) = mspc::channel();
let tx1 = tx.clone(); // this will transmit message to same reciever rx
```
