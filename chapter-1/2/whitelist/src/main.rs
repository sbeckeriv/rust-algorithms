mod whitelist;
use std::io;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
//page 99
//printf one | cargo run two
fn main() {
    // is there a better way to do this?
    let mut arguments: Vec<String> = env::args().collect();
    let file_string = arguments.pop().unwrap();
    // a better way to read a files content?
    let file = Path::new(&file_string);
    let mut open_file = File::open(&file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    let mut num_array = buffer.lines().map(|num| num.parse::<u64>().unwrap()).collect();
    let whitelist = whitelist::StaticSetOfInts::new(num_array);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.parse::<u64>() {
                Ok(num) => {
                    if !whitelist.contains(num) {
                        println!("{}", input);
                    }
                }
                Err(num) => println!("Could not parse:{:?}", num),
            }
        }
        Err(error) => {}
    }
}
