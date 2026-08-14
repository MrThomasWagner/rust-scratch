
fn main() {
    let v = [1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");
    println!("{v:?}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    if let Some(x) = v.get(5) {
        println!("The fifth element is {x}");
    } else {
        println!("There is no sixth element");
    }

    let v2 = [100, 32, 97];
    for i in v2 {
        println!("{i}");
    }
    println!("{v2:?}");

    let v3 = vec![100, 32, 97];
    for i in &v3 {
        println!("{i}");
    }
    println!("{v3:?}");

    let mut v4 = [100, 32, 97];
    for i in &mut v4 {
        println!("{i}");
    }
    println!("{v4:?}");
    // let mut v2: Vec<u8> = Vec::new();
    // v2.push(0);
    // v2.push(1);
    // v2.push(2);
    // println!("v2 time: {v2:?}");
    // let first = &v2[0]&&&&;
    // println!("{first:?}");
    // let second = v2[1];
    // println!("{second:?}");

    println!();
    enum_vecs();
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vecs() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{row:?}");
}
