fn main() {
    println!("Hello, world!");
    first_hashmap();
    word_counter();
    let list = [1,2,3,4,5,6,7,8,9,10].to_vec();
    let m: u32 = median(&list);
    println!("median of {list:?} is {m}");
    println!("{m:?}"); 
    let idk: String = quiet_piggy("calculate");
    println!("{idk}");
}

fn first_hashmap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");
    println!();



    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {scores:?}");
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

// fn intricate(){
//     use std::collections::HashMap;
//
//     let mut field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");
//
//     let mut map = HashMap::new();
//     map.insert(field_name, &field_value);
//     // field_name and field_value are invalid at this point, try using them and
//     // see what compiler error you get!
//     // field_name = "this wont work".to_string();
//     // println!("{field_name}");
// }

fn word_counter() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn median(list: &Vec<u32>) -> u32 {
    let mut sorted: Vec<u32> = list.to_vec();
    sorted.sort();
    let i = sorted.len() / 2;
    sorted[i]
}

fn quiet_piggy(s: &str) -> String {
    let as_str = s.to_string();
    let rest = as_str.chars().skip(1).collect::<String>();
    let start = as_str.chars().next().unwrap().to_string();
    format!("{rest}{start}ay")
}










