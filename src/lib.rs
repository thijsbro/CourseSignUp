


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

struct Child {
    pub first_name: String,
    pub last_name: String,
    pub class: String,
    pub course_choice: Vec<course>,
}

iml Child {
    pub fn new{child_data: &[String]) -> Result<Config, &'static str> {
        if args.len() < 6 {
            return  Err("not enough arguments");
        }
        if args.len() > 6 {
            return Err("too many arguments");
        }
    }
}



fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
