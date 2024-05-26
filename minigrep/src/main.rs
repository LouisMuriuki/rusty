use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let contents = fs::read_to_string(&config.file_path).expect("should have read the file");
    println!("Searching for {:?}", config.query);
    println!("In file {:?}", &config.file_path);
    println!("Contents {:?}", &contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
