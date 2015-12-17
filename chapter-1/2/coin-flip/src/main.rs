extern crate rand;

#[derive(Debug)]
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
use rand::distributions::{IndependentSample, Range};
fn main() {
    let mut counters: Vec<Counter> = vec![Counter::new("1"),
                                          Counter::new("2"),
                                          Counter::new("3"),
                                          Counter::new("4"),
                                          Counter::new("5"),
                                          Counter::new("6")];
    let random_door = Range::new(0, 6);

    let mut rng = rand::thread_rng();
    if rng.gen() {
        println!("i32: {}", random_door.ind_sample(&mut rng));
    }
    for x in 0..500 {
        counters[random_door.ind_sample(&mut rng)].increment();
    }
    println!("{:?}", counters);
}
