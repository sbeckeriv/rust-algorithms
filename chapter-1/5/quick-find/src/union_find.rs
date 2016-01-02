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
    pub fn union(&mut self, left: usize, right: usize) {
        let left_id = self.find(left);
        // println!("found left {}", left_id);
        let right_id = self.find(right);
        // println!("found right {}", right_id);
        if left_id != right_id {
            for i in 0..self.ids.len() {
                if self.ids[i] == left_id {
                    self.ids[i] = right_id.clone();
                }
            }
            self.count -= 1;
        }
    }

    pub fn find(&self, id: usize) -> usize {
        let x = self.ids[id];
        // println!("finding {} {}", id, self.ids.len());
        x
    }
    pub fn connected(&self, left: usize, right: usize) -> bool {
        let x = self.find(left) == self.find(right);
        // println!("connected {}", x);
        x

    }
}
