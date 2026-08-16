use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    match read_file_to_string("./src/main.rs") {
        Ok(content) => println!("\nFile content: \n\n {content}"),
        Err(e) => panic!("could not read file! {e}"),
    };
}
fn read_file_to_string(n: &str) -> Result<String, io::Error> {
    let mut file_contents = String::new();
    File::open(n)?.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn idiomatic() -> Result<String, io::Error> {
    fs::read_to_string("src/main.rs")
}
// let greeting_file_result = File::open("hello_there.txt");
//
// let greeting_file = match greeting_file_result {
//     Ok(f) => f,
//     Err(e) => match e.kind() {
//        std::io::ErrorKind::NotFound => match File::create_new("hello_there.txt") {
//            Ok(r) => r,
//            Err(e) => panic!("Uncreated file create failed: {e}"),
//        },
//        _ => {
//            panic!("Unhandlable error: {e}");
//        }
//     }
// };
//
// fn with_helper_nmethod () {
//     let f = File::open("somefile.txt").expect("somefile.txt needs to exist.");
// }
