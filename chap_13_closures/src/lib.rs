use std::{thread, time::Duration};

pub fn scratch() {
    let expensive_closer = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_millis(500));
        num
    };

    let result = expensive_closer(15);
    println!("Result: {result}");
    println!();
}

pub fn closure_types() {
    let c1 = |x: u32| -> u32 { x + 2 };
    println!("c1: {}", c1(2));

    let c2 = |x| x + 2; // |x| { x + 2 }
    println!("c2: {}", c2(2));

    let c3 = |x| x + 2;
    println!("c3: {}", c3(2));
}

pub fn closure_typing() {
    let c1 = |x| x;
    println!("Result is {}!", c1(String::from("fantastic!")));
    // println!("Result is {}!", c1(5)); cannot call with integer as prev line is type-binding
}

pub fn closure_borrowing() {
    println!("Borrows ----------");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

pub fn closure_mutables() {
    println!("Mutables ----------");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut adds_to = || list.push(4);

    // println!("Before calling closure: {list:?}"); // cannot borrow immutably a mutable value
    adds_to();
    println!("After calling closure: {list:?}");
}

pub fn thread_moving() {
    println!("Threads ---------------");
    let list = vec![1, 2, 3];
    println!("Before defining the closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle{
    width: u32, 
    // length: u32,
}

pub fn sorting_by_closure() {
    println!("Sorting by closure -------------");
    let mut list = [
        Rectangle {width: 10},
        Rectangle {width: 1},
        Rectangle {width: 7},
    ];

    list.sort_by_key(|r| r.width);
    println!("sorted: {list:#?}");
}

#[cfg(test)]
mod tests {
    use crate::{closure_borrowing, closure_mutables, closure_types, closure_typing, scratch, sorting_by_closure, thread_moving};

    #[test]
    fn idk() {
        scratch();
        closure_types();
        closure_typing();
        closure_borrowing();
        closure_mutables();
        thread_moving();
        sorting_by_closure();
    }
}
