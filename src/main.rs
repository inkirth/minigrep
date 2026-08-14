use std::{env, error::Error, fs, process};
use minigrep::{search, search_case_insensitive};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });


    if let Err(e)=run(config){
        println!("Application error: {e}");
        process::exit(1);    

    }
}

fn run(config:Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents){
            println!("{line}");
        }
    }else {
        for line in search(&config.query, &contents){
            println!("{line}");
        }
    };

    Ok(())
}


`pub struct Config{
    pub query: String,
    pub file_path : String,
    pub ignore_case: bool,
}

impl Config{
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query: String = match args.next(){
            None => return Err("Didn't get a query string"),
            Some(arg) => arg
        };
        
        let file_path: String = match args.next(){
            None => return Err("Didn't get a file path"),
            Some(arg) => arg
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query, 
            file_path, 
            ignore_case,
        })
    }
}

