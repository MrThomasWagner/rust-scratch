// fn main() {
//     let width = 30;
//     let height = 50;
//
//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(width, height)
//     );
// }
//
// fn area(width: i32, height: i32) -> i32 {
//     width * height
// }

// fn main() {
//     let rect1  = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     let a = area(&rect1);
//     println!("area of rect1: {}", a);
//     println!(
//         "Original height: {}; Original width: {}",
//         rect1.height,
//         rect1.width
//     );
//
//     println!();
//     println!("All of rect1: {:#?}", rect1);
// }
//
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
//
// #[derive(Debug)]
// struct Rectangle {
//     width: u32, 
//     height: u32,
// }

fn main() {
    let rect1  = Rectangle {
        width: 30,
        height: 50,
    };

    let a = rect1.area();

    println!("area of rect1: {}", a);
    println!(
        "Original height: {}; Original width: {}",
        rect1.height,
        rect1.width
    );

    println!();
    println!("All of rect1: {:#?}", rect1);

    let unit = Rectangle::square(1);
    let double_unit = Rectangle::square(2);

    println!();
    println!("unit: {:?}", unit);
    println!("double: {:?}", double_unit);
    println!();
    println!("double can hold single?: {}", double_unit.can_hold(&unit));
    println!("double can hold single?: {}", unit.can_hold(&double_unit));
}

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(l: u32) -> Rectangle {
        Rectangle {
            width: l,
            height: l,
        }
    }
}






