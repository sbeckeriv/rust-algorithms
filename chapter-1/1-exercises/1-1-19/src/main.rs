extern crate time;

fn main() {
    for i in 0..95 {
        // let start = time::now();
        // let x=fib(i);
        // println!("run fib {:?}-{:?} took: {:?}", i,x, time::now()-start);
        let start2 = time::now();
        let y = fib2(i);
        println!("run fib2 {:?}-{:?} took: {:?}", i, y, time::now() - start2);
    }
}


fn fib2(a: isize) -> i64 {
    if a < 3 {
        return 1;
    } else {
        let mut fib1: i64 = 1;
        let mut fib2: i64 = 2;
        let mut temp: i64 = 1;
        for i in 3..a {
            temp = fib1 + fib2;
            fib1 = fib2;
            fib2 = temp;
        }
        temp
    }
}

fn fib(a: isize) -> i64 {
    match a {
        0 => 0,
        1 => 1,
        _ => fib(a - 1) + fib(a - 2),
    }
}
