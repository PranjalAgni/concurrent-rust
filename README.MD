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

- Mutexes in Rust

* What are Mutexes?
  For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one microphone. Before a panelist can speak, they have to ask or signal that they want to use the microphone. When they get the microphone, they can talk for as long as they want to and then hand the microphone to the next panelist who requests to speak. If a panelist forgets to hand the microphone off when they’re finished with it, no one else is able to speak. If management of the shared microphone goes wrong, the panel won’t work as planned!

* First Principles of Mutexes

  1. You must attempt to acquire the lock before using the data

  2. When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

* API of mutexes

  Creating a Mutex

  ```rs
  use std::sync::Mutex;
  fn main() {
      let m = Mutex::new(5);
      {
          let mut num = m.lock().unwrap();
          *num = 6;
      }
      // mutexes unlocks as soon as it goes out of scope
      println!("m = {:?}", m);
  }
  ```
