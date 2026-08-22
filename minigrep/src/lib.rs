pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents.lines().filter(move |l| l.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    // let mut results = Vec::new();
    // let query = &query.to_lowercase();
    //
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    let query = query.to_lowercase();

    contents
        .lines()
        .filter(move |l| l.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results: Vec<&str> = search(query, contents).collect();
        assert_eq!(vec!["safe, fast, productive."], results)
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results: Vec<&str> = search_case_insensitive(query, contents).collect();
        assert_eq!(vec!["Rust:", "Trust me."], results)
    }
}
