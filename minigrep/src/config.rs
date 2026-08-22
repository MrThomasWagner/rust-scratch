use std::env;

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
} 
 
impl Config {
    // pub(crate) fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("Not enough arguments");
    //     }
    //
    //     Ok(Config {
    //         file_path: args[2].clone(),
    //         query: args[1].clone(),
    //         ignore_case: env::var("IGNORE_CASE").is_ok(),
    //     })
    // }
    
    pub(crate) fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
