#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Parser)]
struct CLI {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
 
#[derive(Debug)]
struct CustomError(String);

fn main() -> std::io::Result<()> {
    let args = CLI::parse();

    let file = File::open(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{}", args.path, err)))?;
    let mut reader = BufReader::with_capacity(200, file);

    let mut line = String::new();
    
    while !line.contains(&args.pattern) {
        line.clear();
        let len = reader.read_line(&mut line)?;
    }    

    println!("{}", line);

    Ok(())
}

