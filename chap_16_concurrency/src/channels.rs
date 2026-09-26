use std::{sync::mpsc, thread, time::Duration};

pub fn channeling() {
    println!("VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec!["1", "2", "3", "4"];

        for value in vals {
            println!("About to send a '{}' ...", value);
            thread::sleep(Duration::from_millis(500));
            let val = String::from(value);
            tx.send(val).unwrap();
        }
    });
    
    thread::spawn(move || {
        let vals = vec!["one", "two", "three", "four"];

        for value in vals {
            println!("About to send a '{}' ...", value);
            thread::sleep(Duration::from_millis(500));
            let val = String::from(value);
            tx1.send(val).unwrap();
        }
    });

    // let rec = rx.recv().unwrap();

    for v in rx {
        println!("Received a '{}'", v);
    }
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_out_threads() {
        channeling();
    }
}
