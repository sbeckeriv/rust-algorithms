use std::io;
pub struct StdinLines {
    count: i32,
}

impl StdinLines {
    pub fn new() -> StdinLines {
        StdinLines { count: 0 }
    }
}

impl Iterator for StdinLines {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.count = self.count + 1;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => Some(input),
            Err(_) => None,
        }
    }
}
