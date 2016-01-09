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
        if low >= high {
            return;
        }
        let j = self.partition(low, high);
        self.sort(low, j - 1);
        self.sort(j + 1, high);
    }

    fn partition(&mut self, lo: usize, hi: usize) -> usize {
        println!("low:{} high:{}", lo, hi);
        println!("{:?}", self.vec);
        let mut i = lo;
        let mut j = hi + 1;
        let mut v = self.vec[i];
        loop {
            loop {
                i += 1;
                if i >= hi || self.vec[i] < v {
                    break;
                }
            }

            loop {
                j -= 1;
                if j <= lo || v < self.vec[j] {
                    break;
                }
            }
            if i >= j {
                break;
            }
            self.vec.swap(j, i);
        }
        println!("lo{} j{}", lo, j);
        self.vec.swap(lo, j);
        j

    }
}
