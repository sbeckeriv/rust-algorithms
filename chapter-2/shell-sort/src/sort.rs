#[derive(Debug)]
pub struct Algo<'a> {
    vec: &'a mut Vec<usize>,
}

impl<'a> Algo<'a> {
    pub fn new(vec: &'a mut Vec<usize>) -> Self {
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
        println!("low:{} high:{}", low, high);
        if high > low {
            let j = self.partition(low, high);
            self.sort(low, j - 1);
            self.sort(j + 1, high);
        }
    }

    fn partition(&mut self, lo: usize, hi: usize) -> usize {
        println!("par:: low:{} high:{}", lo, hi);
        let mut i = lo;
        let mut j = hi;
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