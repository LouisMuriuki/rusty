use std::{env, process};
use minigrep::{Config,run};
fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprint!("Problem parsing the arguments{}", err);
        process::exit(2);
    });

    println!("The query is {:?}", config.query);
    println!("The filepath is {:?}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application Errror{:?}", e);
        process::exit(1);
    }
    // run(config).unwrap_or_else(|e|{
    //     dbg!(e);
    //     process::exit(1)
    // })
}


