fn main() {
    println!("Hello, world!");
    let w = Message::Write(String::from("A message to you, Rudy"));
    println!("{w:?}");

    let q = Message::Quit();
    println!("{q:?}");

    let cc = Message::ChangeColor(RGB {
        r: 10,
        g: 11,
        b: 12,
    });
    println!("{cc:?}");

    let m = Message::Move { x: 10, y: 15 };
    println!("{m:?}");

    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let vt_quarter = Coin::Quarter(UsState::Vermont);
    let ma_quarter = Coin::Quarter(UsState::Massachusetts);

    println!("Quarter straight debug: {vt_quarter:?}");
    println!("{}", flip(p));
    println!("{}", flip(n));
    println!("{}", flip(d));
    println!("{}", flip(vt_quarter));
    println!("{}", flip(ma_quarter));

    let some_three = Some(3);
    let some_four = Some(4);
    let some_five = Some(5);
    number_match(some_three);
    number_match(some_four);
    number_match(some_five);

    if let Some(5) = some_four {
        println!("this is four!");
    } else {
        println!("that was NOT a four");
    }

    if let Some(5) = some_five {
        println!("this is five!");
    } else {
        println!("that was NOT a five");
    }
}

fn number_match(some_three: Option<i32>) {
    match some_three {
        Some(3) => println!("three!"),
        _ => println!("three wouldve been a magic number"),
    }
}

#[derive(Debug)]
enum Message {
    Quit(),
    Write(String),
    ChangeColor(RGB),
    Move { x: u32, y: u32 },
}

#[derive(Debug)]
struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

impl Message {
    fn foo(&self) {
        match self {
            Message::Quit() => println!("idk!"),
            Message::Write(_) => println!("idk!"),
            Message::ChangeColor(_) => println!("idk!"),
            Message::Move { x, y } => println!("idk!"),
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn flip(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_state) => {
            println!("This is from {us_state:?}, nice!");
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Vermont,
    NewHampshire,
    Maine,
    Massachusetts,
    Connecticut,
    RhodeIsland(),
}
