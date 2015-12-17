extern crate rand;
struct Counter {
    name: String,
    count: i64,
}

impl Counter {
    fn new(name: &str) -> Counter {
        Counter {
            name: name.to_string(),
            count: 0,
        }
    }
    fn increment(&mut self) -> i64 {
        self.count = self.count + 1;
        self.count
    }
}

use rand::Rng;
fn main() {
    let mut counter = Counter::new("heads");
    println!("{:?}", counter.increment());

    let mut rng = rand::thread_rng();
    if rng.gen() {
        // random bool
        println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
    }

}
