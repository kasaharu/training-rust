fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v is {:?}", v);

    let v2 = vec![1, 2, 3];
    println!("v2 is {:?}", v2);

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("v3 is {:?}", v3);
}
