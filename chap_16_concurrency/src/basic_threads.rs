use std::{thread, time::Duration};

pub fn basic_thread_spawn() {
    let join = thread::spawn(|| {
        for i in 1..3 {
            println!("hello {i} from a spawned thread!");
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..2 {
        println!("hello {i} from the main thread!");
        thread::sleep(Duration::from_millis(50));
    }

    join.join().unwrap();
}

pub fn passing_variables() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here is a vector dont ya know: {v:?}");
    });

    handle.join().unwrap();
}

