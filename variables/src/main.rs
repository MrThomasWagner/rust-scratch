use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y: u8 = 255;
    println!("The value of y is {}", y);
    // let y = y + 1;
    // y = 3;
    println!("The value of y is {}", y);

    let tup: (u8, f64, i8) = (10, 1.5, -1);
    let (j, k, _) = tup;
    println!("j: {}, k: {}, l: {}", j, k, tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a.len());

    // index_eg();
}

fn index_eg() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
