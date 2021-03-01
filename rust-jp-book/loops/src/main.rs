fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number2 in (1..4).rev() {
        println!("{}!", number2);
    }

    println!("LIFTOFF!!!");
}
