fn main() {
    let s1 = String::new();
    println!("s1 is {}", s1);

    let data = "initial contents";
    let s2 = data.to_string();
    println!("s2 is {}", s2);

    let s3 = String::from("initial contents");
    println!("s3 is {}", s3);
}
