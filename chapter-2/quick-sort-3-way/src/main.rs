mod timer;
mod sort;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

fn read_file_chars(file_string: String) -> Vec<isize> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>()
}

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    let file_string = arguments.pop().unwrap();
    let mut vec = read_file_chars(file_string);
    let len = vec.len()-1;
    let mut sorter = sort::Algo::new(&mut vec);
    let spent = timer::record(|| {
        sorter.sort(0, len);
    });
    if len<50_000{
        println!("{:?}", sorter);
    }
    println!("{:?}", sorter.is_sorted());
    println!("{:?}", spent);
}
