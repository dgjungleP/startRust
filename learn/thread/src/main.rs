use std::{sync::mpsc, thread, time::Duration};

fn main() {
    println!("Hello, world!");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let recived = rx.recv().unwrap();
    println!("Got: {}", recived);
}
