mod timer;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
fn count(array: Vec<isize>) -> usize {
    let mut count = 0;
    let array_len = array.len();
    for i in 0..array_len {
        for f in i + 1..array_len {
            match array.binary_search(&(0 - array[i] - array[f])) {
                Ok(n) => {
                    if n > f {
                        count += 1;
                    }
                }
                Err(_) => {}
            }
        }
    }
    count
}

fn read_ints(file_string: String) -> Vec<isize> {
    let file = Path::new(&file_string);
    let mut open_file = File::open(file).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    buffer.lines()
        .map(|num| num.trim().parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    let file_string = arguments.pop().unwrap();
    let spent = timer::record(|| {
        let mut x = read_ints(file_string);
        x.sort();
        println!("{:?}",count(x));
    });
    println!("{:?}", spent);

}
