#[derive(Debug)]
pub struct Algo<'a, T: 'a> {
    vec: &'a mut Vec<T>,
    count: usize,
}
impl<'a, T: Ord> Algo<'a, T> {
    pub fn new(vec: &'a mut Vec<T>) -> Self {
        Algo {
            vec: vec,
            count: 0,
        }
    }
    pub fn len(&self) -> usize {
        3
    }
    pub fn is_sorted(&self) -> bool {
        let mut sorted = true;
        for i in 0..self.len() {
            if self.vec[i] > self.vec[i + 1] {
                sorted = false;
            }
        }
        sorted
    }

    pub fn swap(&self, t: usize, j: usize) {}
}
