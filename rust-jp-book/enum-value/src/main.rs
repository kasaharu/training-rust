enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor,
}

impl Message {
    fn call(&self) {
        println!("print self");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number is {:?}", some_number);
    println!("some_string is {:?}", some_string);
    println!("absent_number is {:?}", absent_number);
}
