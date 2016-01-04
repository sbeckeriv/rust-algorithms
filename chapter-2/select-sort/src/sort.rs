#[derive(Debug)]
pub struct Algo {
    vec: Vec<String>,
}
impl Algo {
    pub fn new(vec: Vec<String>) -> Self {
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
    pub fn sort(&mut self) {
        for i in 0..self.vec.len() {
            let mut min = i;
            for j in i + 1..self.vec.len() {
                if self.vec[j] < self.vec[min] {
                    min = j;
                }
            }
            self.vec.swap(i, min);
        }
    }
}
