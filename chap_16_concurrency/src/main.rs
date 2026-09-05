use std::{thread, time::Duration};

fn main() {
    basic_thread_spawn();
    passing_variables();
}

fn basic_thread_spawn() {
    let join = thread::spawn(|| {
        for i in 1..10 {
            println!("hello {i} from a spawned thread!");
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("hello {i} from the main thread!");
        thread::sleep(Duration::from_millis(500));
    }

    join.join().unwrap();
}

fn passing_variables() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Here is a vector dont ya know: {v:?}");
    });

    handle.join().unwrap();
}
