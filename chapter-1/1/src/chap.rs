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
    if(n<2){
        false
    } else{
        for count in 2..(n/2){
            if(n%count==0){
                return false;
            }
        }
        true
    }
}

use std::f64;
pub fn sqrt(n: f64)-> f64{
    if(n<0.0){
        f64::NAN
    }else{
        let mut t= n.clone();
        let base = 10.0_f64;
        let err = base.powi(-15);
        while ((t-(n/t)).abs() > (err*t)) {
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
