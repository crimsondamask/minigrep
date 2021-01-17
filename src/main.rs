use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::io::prelude::*;
use std::env;


fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for: {}", config.query);
    println!("Searching in: {}", config.filename);
    // println!("Contents in: {}", read_file(&config.filename));

    run(config);
    
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) {
    let mut f = File::open(config.filemame).expect("File not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("Couldn't read file");

    println!("Contents =======>> \n{}", contents);
}
