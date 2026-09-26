use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn multi_mutex() {
    println!("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT");
    let counter = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num *= 2;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    println!("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT");
}

pub fn mutex_it() {
    println!("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
    println!("WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_mutex() {
        mutex_it();
        multi_mutex();
    }
}
