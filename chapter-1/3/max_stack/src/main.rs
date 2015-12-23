use std::io;
mod stackable;
// I skipped page 133 and just did the generic version
fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let mut stack = stackable::MaxStack::<&str>::new(100);
    for word in string.split_whitespace() {
        if word != "-" {
            match stack.push(word) {
                Ok(_) => {}
                Err(_) => panic!("Stack is not large enough"),
            }
        } else {
            match stack.pop() {
                Some(out) => print!("{} ", out),
                None => println!("Stack empty"),
            }
        }
    }
    println!("{} left on stack", stack.size())

}
