#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, world!");
    println!(
        "Quarter value is {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );
    println!("Dime    value is {}", value_in_cents(Coin::Dime));
    println!("Nickel  value is {}", value_in_cents(Coin::Nickel));
    println!("Penny   value is {}", value_in_cents(Coin::Penny));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six  value is {:?}", six);
    println!("none value is {:?}", none);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
