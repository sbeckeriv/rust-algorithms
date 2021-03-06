extern crate algorithms_helpers;
use algorithms_helpers::{timer, stdin_lines};
mod sort;
mod pq;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Transaction(u64, String);

fn read_file_lines(file_string: String) -> Vec<Transaction> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.lines()
        .map(|num| {
            let temp: Vec<&str> = num.rsplitn(2, " ").collect();
            let num: u64 = temp[0].parse::<u64>().unwrap();
            Transaction( num,temp[1].trim().to_string())
        })
    .collect::<Vec<Transaction>>()
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
    let mut vec = read_file_lines(file_string);
    let len = vec.len() - 1;
    let mut empty_vec: Vec<Transaction> = Vec::new();
    empty_vec.push(Transaction(0, "trash".to_string())); // figure out how to remove this
    let mut sorter = pq::PQAlgo::<Transaction>::new(&mut empty_vec);
    if len<50_000{
        println!("{:?}", sorter);
    }
    let spent = timer::record(|| {
        for i in vec{
            sorter.insert(i);
        }
        println!("pre-{:?}", sorter);
        sorter.sort();
        println!("{:?}", sorter);
        for i in 0..len+1{
            let x = sorter.del_max();
        }
    });
    println!("{:?}", spent);
    if len<50_000{
        println!("{:?}", sorter);
    }

}
