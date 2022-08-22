use std::env;
use std::fs;
use std::path::Path;
use std::process;

use subtext::block::Block;
use subtext::parse;
use subtext::primitive::Entity;
use tendril::fmt::Slice;

pub fn main() {
    let args = env::args();

    if args.len() < 2 {
        println!("Please provide the path to a .subtext file!");
        std::process::exit(1);
    }

    let path = env::args().last().unwrap();
    let current_dir = env::current_dir().unwrap();
    let full_file_path = current_dir.join(Path::new(&path));

    println!("Attempting to parse {:?}", full_file_path);
    match fs::read(full_file_path) {
        Ok(buffer) => {
            // let result = parse(buffer.as_bytes());
            let result: Vec<Block<Entity>> = parse(buffer.as_bytes()).unwrap().collect();

            println!("Parsed blocks: {:#?}", result);
        }
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    }
}
