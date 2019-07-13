enum IPAddressKind {
    V4,
    V6
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

#[derive(Debug)]
enum USState {
    Arizona,
    Alabama,
    Massachusetts,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn main() {
    // let v4 = IPAddressKind::V4;
    // value_in_cents(Coin::Quarter(USState::Massachusetts));
    let two = plus_one(1);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Other number");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
