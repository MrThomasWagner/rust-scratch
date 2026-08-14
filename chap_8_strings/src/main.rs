fn main() {
    println!("Hello, world!");
    string_example();
}

fn string_example() {
    let s = "a string";
    let r: &str = s;
    let st: String = "another string".to_string();
    println!("{}", s);
    println!("{}", r);
    println!("{}", st);
}
