fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1);
    // println!("s1 = {}", s1); // 所有権が移動しているため s1 は有効ではない

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
