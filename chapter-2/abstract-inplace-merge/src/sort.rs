#[derive(Debug)]
pub struct Algo {
    vec: Vec<String>,
    aux: Vec<String>,
}
impl Algo {
    pub fn new(vec: Vec<String>) -> Self {
        Algo {
            vec: vec,
            aux: Vec::new(),
        }
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
    fn merge(&mut self, lo: usize, mid: usize, hi: usize) {
        let mut i = lo;
        let mut j = mid + 1;
        for k in lo..hi {
            self.aux[k] = self.vec[k].clone();
        }

        for k in lo..hi {
            if i > mid {
                self.vec[k] == self.aux[j].clone();
                j += 1;
            } else if j > hi {
                self.vec[k] == self.aux[i].clone();
                i += 1;
            } else if self.aux[j] < self.aux[i] {
                self.vec[k] = self.aux[j].clone();
                j += 1;
            } else {
                self.vec[k] == self.aux[i].clone();
                i += 1;
            }
        }

    }
    pub fn sort(&mut self) {
        let n = self.vec.len();
        let mut h = 1;
        loop {
            if h >= n / 3 {
                break;
            }
            h = 3 * h + 1;
        }
        while h >= 1 {
            for i in h..n {
                let mut j = i;
                while self.vec[j] < self.vec[j - h] {
                    self.vec.swap(j, j - h);
                    j -= h;
                    if j < h {
                        break;
                    }
                }
            }
            h = h / 3;
        }

    }
}
