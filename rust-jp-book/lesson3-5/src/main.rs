fn main() {
    let number = fibonacci(15);
    println!("{}", number);
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
