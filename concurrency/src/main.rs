use std::thread;
use std::time::Duration;

fn main() {
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
