pub struct UF {
    pub count: usize,
    ids: Vec<usize>,
    size: Vec<usize>,
}

impl UF {
    pub fn new(size: &usize) -> Self {
        let count = size.clone();
        UF {
            ids: (0..count).collect(),
            size: vec![1; count],
            count: count,
        }
    }

    pub fn union(&mut self, left: usize, right: usize) {
        let left_id = self.find(left);
        // println!("found left {}", left_id);
        let right_id = self.find(right);
        // println!("found right {}", right_id);
        if left_id != right_id {
            if self.size[left_id] > self.size[right_id] {
                self.ids[left_id] = right_id.clone();
                self.size[right_id] += self.size[left_id];
            } else {
                self.ids[right_id] = left_id.clone();
                self.size[left_id] += self.size[right_id];
            }
            self.count -= 1
        }
    }

    pub fn find(&self, id: usize) -> usize {
        let mut index = id.clone();
        while index != self.ids[index] {
            index = self.ids[index];
        }
        // println!("finding {} {}", id, self.ids.len());
        index
    }
    pub fn connected(&self, left: usize, right: usize) -> bool {
        let x = self.find(left) == self.find(right);
        // println!("connected {}", x);
        x
    }
}
