//page 23

pub fn abs(x: i64) -> i64{
    if x<0{
        x*(0-1)
    } else{
        x
    }
}

pub fn abs_float(x: f64) -> f64{
    if x<0.0{
        x*(0.0-1.0)
    } else{
        x
    }
}

pub fn is_prime(n: u64) -> bool {
    if n<2 {
        false
    } else{
        for count in 2..(n/2){
            if n%count==0 {
                return false;
            }
        }
        true
    }
}

use std::f64;
pub fn sqrt(n: f64)-> f64{
    if n<0.0 {
        f64::NAN
    }else{
        let mut t= n.clone();
        let base = 10.0_f64;
        let err = base.powi(-15);
        while (t-(n/t)).abs() > (err*t) {
            t=(n/t+t)/2.0;
        }
        t
    }
}


pub fn hypo(a: f64, b: f64) -> f64 {
    (a*a+b*b).sqrt()
}

pub fn h(n: u64) -> f64{
    let mut sum = 0.0;
    for i in 1..n+1{
        sum = sum+(1.0/i as f64)
    }
    sum
}

//page #25. close enough.
//I would return a option instead of a bool.
pub fn rank(vec: &Vec<usize>, find: usize) -> bool{
    let max = vec.len();
    rank_find(vec, find, 0, max)
}

fn rank_find(vec: &Vec<usize>, find: usize, min: usize, max: usize) -> bool{
    let mid = min+((max-min)/2);
    let mid_value = vec[mid];
    if max==min {
        false
    }else if mid_value == find {
        true
    }else if mid_value < find {
        rank_find(vec, find, mid, max)
    }else{
        rank_find(vec, find, min, mid)
    }
}
//page # 32
extern crate rand;
use self::rand::Rng;
pub fn uni1(a: u64, b: u64) -> u64{
     a + (rand::random::<f64>()*(b-a) as f64) as u64
}

pub fn uni2(a: u64) -> u64{
     (rand::random::<f64>()*a as f64) as u64
}

pub fn discrete(a: u64){
    //nah
}

// not the same as the book
pub fn shuffle(a: &mut Vec<usize>){
    let mut rng = rand::thread_rng();
    let max = a.len();
    for i in 0..max{
        let swapper = rng.gen::<usize>()%max;
        a.swap(i,swapper);
    }
}
