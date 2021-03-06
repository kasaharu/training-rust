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

    let v4 = vec![1, 2, 3, 4, 5];
    let third1: &i32 = &v4[2];
    let third2: Option<&i32> = v4.get(2);
    let ten: Option<&i32> = v4.get(9);
    println!("third1 is {:?}", third1);
    println!("third2 is {:?}", third2);
    println!("ten    is {:?}", ten);

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row is {:?}", row);
}
