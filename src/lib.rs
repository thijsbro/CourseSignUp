

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


pub struct Child {
    pub first_name: String,
    pub last_name: String,
    pub class: String,
    pub course_choice1: String,
    pub course_choice2: String,
    pub course_choice3: String,
}

impl Child {
    pub fn new(child_data: &[String]) -> Result<Child, &'static str> {
        if child_data.len() < 6 {
            return Err("not enough arguments");
        }
        if child_data.len() > 6 {
            return Err("too many arguments");
        }
        let first_name = child_data[0].clone();
        let last_name = child_data[1].clone();
        let class = child_data[2].clone();
        let course_choice1 = child_data[3].clone();
        let course_choice2 = child_data[4].clone();
        let course_choice3 = child_data[5].clone();

        Ok(Child { first_name, last_name, class, course_choice1, course_choice2, course_choice3 } )
    }
}


