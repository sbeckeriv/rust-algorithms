#[derive(Debug)]
pub struct Algo<'a> {
    vec: &'a mut Vec<isize>,
}

impl<'a> Algo<'a> {
    pub fn new(vec: &'a mut Vec<isize>) -> Self {
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
        if high <= low {
            return;
        }
        let j = self.partition(low, high);
        if j > 1 {
            self.sort(low, j - 1);
        }
        self.sort(j + 1, high);
    }
    fn partition(&mut self, lo: usize, hi: usize) -> usize {
        let mut i = lo;
        let mut j = hi + 1;
        let mut v = self.vec[lo];
        loop {
            loop {
                i += 1;
                if self.vec[i] > v || i == hi {
                    break;
                }
            }

            loop {
                j -= 1;
                if v > self.vec[j] || j == lo {
                    break;
                }
            }
            if i >= j {
                break;
            }
            self.vec.swap(i, j);
        }
        self.vec.swap(lo, j);
        j
    }
}
