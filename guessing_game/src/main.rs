extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let number = rand::thread_rng().gen_range(1, 101);

    //println!("Rng is {}", number);

    

    loop {
        println!("Please input a number :");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line..");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Nice guess!");
                break;
            }
        }
    }
}
