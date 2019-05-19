const _MAX_PTS: u32 = 100_000;

fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {}", x);
    x = 3;
    println!("The value of x is {}", x);
    let _guess: u32 = "42".parse().expect("NAN");

    let (_x, _y, z) = (1, 2.0, 'a');
    println!("Yo mama is {}", z);

//    let arr = ['m', 'm', 'm', 'm', 'm', 'm'];
//    println!("{}", arr[7]);

    hello("There");

    println!("{}", control_flow(true, 5));

    let s = String::from("hello");
    hello2(s);
    println!("{}", s);
}

fn control_flow(b: bool, a: i32) -> i32 {
    if b { -a } else { 0 }
}

fn hello2(a: String) {
    println!("Hello {}", a)
}

fn hello(a: &str) {
    println!("Hello {}", a)
}


