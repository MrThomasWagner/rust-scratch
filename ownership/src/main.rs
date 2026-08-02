fn main() {
    println!("Hello, world!");

    let mut s: String = String::from("string of some sort");
    s.push_str("..i think");

    let test_r = String::from("something.. mutated again");
    println!("test_r: {}", test_r);

    let l = length_of_a_string(&test_r);
    println!("test_r: {}", test_r);
    println!("len: {}", l);

    let mut changed = String::from("Hello");
    change(&mut changed);
    println!("changed? {}", changed);

    let o = give_ownership();
    println!("OH? o: {}", o);

    main2();
}

//fn take_ownership(s: String) {
 //   println!("s owned: {}", s);
//}

fn give_ownership() -> String {
    String::from("new owner?")
}

fn length_of_a_string(str_5: &str) -> usize {
    str_5.len()
}

fn change(s: &mut String) {
    s.push_str(", world!!!");
}

fn main2() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {word}"); // last use of word is HERE
    s.clear(); // this is now FINE
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
