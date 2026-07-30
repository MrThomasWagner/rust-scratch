fn main() {
    let x = 5;

    if x < 5 {
        println!("X less than 5");
    } else {
        println!("X equal to or greater than 5");
    }

    let z = if 5 < 6 {
        "5 less than 6"
    } else {
        "what the heck!"
    };
    println!("{}", z);

    let arr: [u32; 5] = [1, 123, 3, 4, 5];
    for x in arr {
        println!("{}", x);
    }

    println!();

    let arr2 = (1..10).rev();
    for i in arr2 {
        println!("{}", i);
    }

    println!("{}", fibo(10));
}

fn fibo(n: u32) -> u128 {
    if n == 0 {
        return 0;
    }

    fn _fibo(current: u128, prev: u128, i: u32, n: u32) -> u128 {
        if i < n {
            _fibo(current + prev, current, i + 1, n)
        } else {
            current
        }
    }

    _fibo(1, 0, 1, n)
}
