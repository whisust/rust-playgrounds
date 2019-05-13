#![allow(dead_code)]
#![allow(unused_variables)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    channels();
}

fn channels() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
       let val = vec!["this", "is", "a", "teeeeeest"];
        for v in val {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let val = vec!["tattaat", "meeeere", "a", "teeeeeest", "geinwier"];
        for v in val {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for rcv in rx {
        println!("Got {}", rcv);
    }
}


fn concurrent() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} for thread", i);
            thread::sleep(Duration::from_millis(300));
        }
    });

    for i in 1..5 {
        println!("Hi, number {} from the main thread", i);
        thread::sleep(Duration::from_millis(300));
    }

    handle.join().unwrap();
}

fn data_ownership() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..3 {
            println!("Hi, vec is {:?} for thread", v);
            thread::sleep(Duration::from_millis(300));
        }
    });

//    drop(v); oh noooo
    handle.join().unwrap();
}
