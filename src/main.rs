extern crate minigrep;

use std::process;
use std::env;
use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem while parsing arguments: {}", err);

        process::exit(1);
    });
    
    // println!("Contents in: {}", read_file(&config.filename));

    println!("Searching for: {}", config.query);
    println!("Searching in: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
    
}

