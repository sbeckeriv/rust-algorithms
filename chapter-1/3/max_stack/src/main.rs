use std::io;
mod stackable;

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
}
