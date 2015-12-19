use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::Path;

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);
    let out_file = arguments.pop().unwrap();
    let out_path = Path::new(&out_file);
    let mut file_open = File::create(&out_path).unwrap();
    for file in arguments {
        let file_path = Path::new(&file);
        let mut open_file = File::open(&file_path).unwrap();
        let mut buffer = Vec::new();
        open_file.read_to_end(&mut buffer).unwrap();
        file_open.write_all(&buffer).unwrap();
    }
}
