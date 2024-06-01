use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vector = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            vector.push(line)
        }
    }
    vector
    // vec![]
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        println!("{:#?}", args);
        if args.len() < 3 {
            return Err("please make sure to pass enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }
}
