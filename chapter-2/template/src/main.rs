extern crate algorithms_helpers;
use algorithms_helpers::{timer,stdin_lines};
mod sort;
mod pq;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

fn read_file_lines(file_string: String) -> Vec<String> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.lines().map(|num| num.to_string()).collect::<Vec<String>>()
}

fn read_file_chars_as_numbers(file_string: String) -> Vec<isize> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.split_whitespace().map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>()
}

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
    let len = vec.len()-1;
    let mut sorter = sort::Algo::<String>::new(&mut vec);
    let spent = timer::record(|| {
        //sorter.sort(0, len);
    });
    if len<50_000{
        println!("{:?}", sorter);
    }
    println!("{:?}", sorter.is_sorted());
    println!("{:?}", spent);
}
