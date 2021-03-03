fn main() {
    let s1 = gives_ownership();
    takes_ownership(s1);
    // println!("s1 = {}", s1); // 所有権が移動しているため s1 は有効ではない

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);

    let x = 10;
    makes_copy(x);
    println!("x = {}", x);
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
