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
}
