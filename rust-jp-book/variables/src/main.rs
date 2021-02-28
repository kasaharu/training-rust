fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x2 = 5;
    let x2 = x2 + 1;
    let x2 = x2 * 2;
    println!("The value of x2 is: {}", x2);
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The value of guess is: {}", guess);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("The value of sum is: {}, difference is: {}, product is: {}, quotient is: {}, remainder is: {} ", sum, difference, product, quotient, remainder);
}
