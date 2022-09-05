#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();

    v.push(5);

    println!("v = {:?}", v);

    let v2 = vec![1, 2, 3];

    println!("v2 = {:?}", v2);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];

    // let does_not_exist = &v[100];

    let does_not_exist = v.get(100);

    print!("does_not_exist = {:?}", does_not_exist);

    let mut v = vec![100, 32, 57];

    v.push(6);

    let first = &v[0];

    println!("The first element is: {}", first);

    println!("v = {:?}", v);

    for i in &mut v {
        *i += 50;
    }

    println!("v = {:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row = {:?}", row);
}
