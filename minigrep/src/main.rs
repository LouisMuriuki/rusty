use std::{env, process};
use minigrep::{Config,run};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        print!("{}", err);
        process::exit(2);
    });

    println!("The query is {:?}", config.query);
    println!("The filepath is {:?}", config.file_path);
    if let Err(e) = run(config) {
        println!("{:?}", e);
        process::exit(1);
    }
}


