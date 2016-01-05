#[derive(Debug)]
pub struct Algo {
    vec: Vec<usize>,
}

impl Algo {
    pub fn new(vec: Vec<usize>) -> Self {
        Algo { vec: vec }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn is_sorted(&self) -> bool {
        let mut sorted = true;
        for i in 0..self.len() - 1 {
            if self.vec[i] > self.vec[i + 1] {
                sorted = false;
            }
        }
        sorted
    }
    pub fn sort(&mut self, low: usize, high: usize) {
        for i in 1..self.vec.len() {
            println!("e{}", i);
            for j in (0..(i + 1)).rev() {
                let index = j as usize;
                println!("{}", index);
                if (index == 0) || self.vec[index] > self.vec[index - 1] {
                    break;
                }
                self.vec.swap(index, index - 1)
            }
        }
    }
    pub fn partition(&mut self, lo: usize, hi: usize) -> usize {
        let mut i = lo;
        let mut j = hi + 1;
        let mut v = self.vec[lo];
        loop {
            while self.vec[i] < v {
                if i == hi {
                    break;
                }
                i += 1;
            }
            while v < self.vec[j] {
                if j == lo {
                    break;
                }
                j -= 1;
            }
            if i == j {
                break;
            }
            self.vec.swap(i, j);
        }
        self.vec.swap(lo, j);
        j
    }
}
