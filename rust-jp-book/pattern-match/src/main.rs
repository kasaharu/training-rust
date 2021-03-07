enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
    println!("Quarter value is {}", value_in_cents(Coin::Quarter));
    println!("Dime    value is {}", value_in_cents(Coin::Dime));
    println!("Nickel  value is {}", value_in_cents(Coin::Nickel));
    println!("Penny   value is {}", value_in_cents(Coin::Penny));
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
