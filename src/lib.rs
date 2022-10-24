use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Config {
    pub file_path: String,
    pub query: String,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments\nusage: mini-grep <query_string> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { file_path, query})
    }
}

pub fn read_and_search (config: Config) -> io::Result<()> {
    let file = File::open(config.file_path)?;
    let file = BufReader::new(file); 

    for line in file.lines() {
        let line = line.unwrap();
        if line.contains(&config.query) {
            println!("{}", line);
        }
    }
    Ok(())
}