use std::cmp::Ordering;
#[derive(Debug)]
pub struct Algo<'a> {
    vec: &'a mut Vec<isize>,
    count: usize,
}

impl<'a> Algo<'a> {
    pub fn new(vec: &'a mut Vec<isize>) -> Self {
        Algo {
            vec: vec,
            count: 0,
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
    pub fn sort(&mut self, lo: usize, hi: usize) {
        self.count += 1;
        if hi <= lo {
            return;
        }
        let mut lt = lo;
        let mut gt = hi;
        let mut i = lo + 1;
        let mut v = self.vec[lo];
        while i <= gt {
            let compared = self.vec[i].cmp(&v);
            match compared {
                Ordering::Less => {
                    self.vec.swap(lt, i);
                    lt += 1;
                    i += 1;
                }
                Ordering::Greater => {
                    self.vec.swap(i, gt);
                    gt -= 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        if lt > 0 {
            self.sort(lo, lt - 1);
        } else {
            self.sort(lo, 0);
        }
        self.sort(gt + 1, hi);
    }
}
