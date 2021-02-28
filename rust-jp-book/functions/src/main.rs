fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let five = five();
    println!("The value of five is: {}", five);

    let x2 = plus_one(5);
    println!("The value of x2 is: {}", x2);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
