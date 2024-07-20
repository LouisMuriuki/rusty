use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut vector = vec![];
//     for line in contents.lines() {
//         if line.contains(query) {
//             vector.push(line)
//         }
//     }
//     vector
//     // vec![]
// }

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vector = vec![];
    let query_search = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_search) {
            vector.push(line)
        }
    }
    vector
}
// impl Config {
//     pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
//         println!("{:#?}", args);
//         if args.len() < 3 {
//             return Err("please make sure to pass enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         let ignore_case = env::var("IGNORE_CASE").is_ok();
//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
//         })
//     }
// }

impl Config {
    pub fn build( mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
