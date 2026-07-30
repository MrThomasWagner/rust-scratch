fn main() {
    println!("Hello, world!");
    another_function(5, 10);
    expressions_example();
    println!("five? {}", five());

    let z: u32 = 5;
    println!("five? {}", z);
}

fn another_function(i: i32, y: i32) {
    println!("The value of i is: {}", i);
    println!("The value of y is: {}", y);
}

fn expressions_example() {
    let x = 10;
    let y = {
        let x = x + 5;
        println!("inner shadow ends up as: {}", x);
        x + 1
    };

    println!("expression ends up as: {}", y);
    println!("shadow ends up as: {}", x);
}

fn five() -> u32 {
    5
}
