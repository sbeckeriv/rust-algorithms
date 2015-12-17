extern crate rand;

#[derive(Debug)]
struct Counter {
    name: String,
    pub count: i64,
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

use rand::distributions::{IndependentSample, Range};
fn main() {
    let mut rng = rand::thread_rng();
    //arg parsing in 1.1 code
    let t=5;
    let mut heads = Counter::new("heads");
    let mut tails = Counter::new("tails");
    let float_range = Range::new(0.0,1.0);
    for _ in 0..t+1{
        if float_range.ind_sample(&mut rng)>0.5{
            heads.increment();
        }else{
            tails.increment();
        }
    }

    println!("Heads {:?}", heads);
    println!("Tails {:?}", tails);
    println!("delta {:?}", (heads.count-tails.count).abs());


    let mut counters: Vec<Counter> = vec![Counter::new("1"),
                                          Counter::new("2"),
                                          Counter::new("3"),
                                          Counter::new("4"),
                                          Counter::new("5"),
                                          Counter::new("6")];
    let random_dice = Range::new(0, 6);
    for _ in 0..500 {
        counters[random_dice.ind_sample(&mut rng)].increment();
    }
    println!("{:?}", counters);
}
