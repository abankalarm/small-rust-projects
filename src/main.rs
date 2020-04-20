extern crate minidirb;

use std::env;
use std::process;
use minidirb::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let config = Config::new(&args).unwrap_or_else(|err |{
        println!("parsing problem");
        process::exit(1);
    });

    println!("searching for {} in {}", config.query, config.filename);

    if let Err(e) = minidirb::run(config){
        println!("application error");
        process::exit(1);
    }
    
    
    }




