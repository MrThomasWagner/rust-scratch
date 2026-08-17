#[derive(Debug)]
struct Important<'a, 'b> {
    part: &'a str,
    note: &'b str,
}

fn main() {
    let string1 = String::from("string1 is a little longer");
    let string2 = "string2";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{result}'");

    exercise_longest();

    println!();

    let var_name = String::from("important thing");
    let mut i = Important {
        part: &var_name,
        note: "some note or something",
    };

    // { //no work:
    //     let idk = String::from("some other value");
    //     i.part = idk.as_str();
    // }

    println!("{i:?}");

    let longest_result;
    let test1 = String::from("asdfasdf");

    {
        let test2 = String::from("dfdaadfa");
        longest_result = longest2(&test1, &test2);
    }

    println!("Longest result is: {longest_result}");
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

fn longest2<'a, 'b>(string1: &'a str, string2: &'b str) -> &'a str {
    string1
}

fn exercise_longest() {
    let string1 = String::from("this is a pretty long string");

    {
        let string2 = String::from("shorter");
        let result = longest(&string1, &string2);

        println!("The longest string is '{result}'");
    }
}
//
// fn exercise_longest_fail() {
//     let string1 = String::from("this is a pretty long string");
//     let result;
//
//     {
//         let string2 = String::from("shorter");
//         result = longest(&string1, &string2);
//     }
//
//     println!("The longest string is '{result}'");
// }

// fn exercise_longest_fail2() {
//     let string1 = String::from("this is a pretty long string");
//     let string2 = String::from("shorter");
//     let result;
//     {
//         result = longest(&string1, &string2);
//     }
//
//     println!("The longest string is '{result}'");
// }
