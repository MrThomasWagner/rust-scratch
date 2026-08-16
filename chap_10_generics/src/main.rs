mod generic_largest;

fn main() {
    let number_list = [34, 50, 25, 101, 65];
    let number_list2 = [21, 23, 98, 1224, 2132];

    largest(&number_list);
    largest(&number_list2);

    generic_largest::largest(&number_list2);

    let p = Point {
        x: 3.4,
        y: 4,
    };

    let p2 = Point {
        x: 3,
        y: 4,
    };

    let p3 = Point {
        x: p.y / 2, 
        y: p.x / 2.0,
    };

    println!("{p:?}");
    println!("{p2:?}");
    println!("{p3:?}");
    println!("{:?}", p3.mixed_half());
    println!("Many times halved: {:?}", p3.mixed_half().mixed_half().mixed_half().mixed_half().mixed_half());
    println!("Not as many times halved: {:?}", p3.mixed_half().mixed_half());
    println!();
    println!("P3; x: {}, y:{}", p3.x(), p3.y());
}

fn largest(number_list: &[i32]) {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    } 
}

impl Point<i32, f64> {
    fn mixed_half(&self) -> Point<i32, f64> {
        Point {
            x: &self.x / 2,
            y: &self.y / 2.0,
        }
    }
}
