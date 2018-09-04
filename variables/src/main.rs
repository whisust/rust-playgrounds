const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    x = 6;
    println!("x is {}", x);
    println!("you can have a max of {} points", MAX_POINTS);

    let guess: u32 = "42".parse().expect("Not a number");

    let c1: char = '😂';

    println!("Char is {}", c1);

    println!("Plus one is {}", add_count(4, 10));
}

fn add_count(i: u32, count: u32) -> u32 {
    let mut new_i = i;
    for _ in 0..count {
        new_i += 1;
    }
    new_i
}
