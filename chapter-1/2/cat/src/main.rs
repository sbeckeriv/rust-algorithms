use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);
    let out_file = arguments.pop();
    let mut f_open = try!(File::create(out_file));
    try!(f_open.write_all(b"Hello, world!"));

    let mut f = try!(File::open("foo.txt"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    assert_eq!(s, "Hello, world!");
    for file in arguments {
    }
}
