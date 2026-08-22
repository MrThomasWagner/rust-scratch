use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;
use crate::config::Config;

mod config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    // let results = if config.ignore_case {
    //     search_case_insensitive(&config.query, &file_contents)
    // } else {
    //     search(&config.query, &file_contents)
    // };
    // for line in results {
    //     println!("{line}");
    // }

    if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents).for_each(|l| println!("{l}"));
    } else {
        search(&config.query, &file_contents).for_each(|l| println!("{l}"));
    };

    Ok(())
}

