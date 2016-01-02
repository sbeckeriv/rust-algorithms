pub struct UF {
    pub count: usize,
    ids: Vec<usize>,
}

impl UF {
    pub fn new(size: &usize) -> Self {
        let count = size.clone();
        UF {
            ids: (0..count).collect(),
            count: count,
        }
    }
    pub fn union(&mut self) {}
    pub fn find(&self, id: &usize) -> usize {
        3
    }
    pub fn connected(&self, left: &usize, right: &usize) -> bool {
        self.find(&left) == self.find(&right)
    }
}
