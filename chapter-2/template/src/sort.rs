struct Algo {
    vec: Vec<String>,
}
impl Algo {
    pub fn new() -> Self {
        Algo { vec: Vec::new() }
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
