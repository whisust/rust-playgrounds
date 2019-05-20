enum Message {
    Hello { id:u32 },
}

fn main() {
    let x = 0.01;

    match x {
        0.0 ... 10.0  => println!("In range"),
        _ => println!("Not in range...")
    }

    let v = vec![1, 2, 3, 4];
    match v.as_slice() {
        [first, middle, last] => println!("First and last are {}, {}", first, last),
        _ => println!("nieeeh"),
    }

    let msg = Message::Hello {id: 32};
    match msg {
        Message::Hello {id: uuuu@0...42} => println!("well ok {}", uuuu),
        _ => println!("stuff"),
    }
}
