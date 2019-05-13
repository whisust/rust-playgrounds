use std::thread;
use std::time::Duration;

fn main() {
    data_ownership();
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
