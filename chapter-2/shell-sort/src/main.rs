mod timer;
mod sort;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use sort::Algo;

fn read_file_chars(file_string: String) -> Vec<String> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|num| num.to_string()).collect::<Vec<String>>()
}

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    let file_string = arguments.pop().unwrap();
    let mut vec = read_file_chars(file_string);
    let mut sorter = Algo::new(vec);
    let spent = timer::record(|| {
        sorter.sort();
    });
    println!("{:?}", sorter);
    println!("{:?}", sorter.is_sorted());
    println!("{:?}", spent);
}
