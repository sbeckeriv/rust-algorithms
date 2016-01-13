
#[derive(Debug)]
pub struct PQAlgo<'a, T: 'a> {
    vec: &'a mut Vec<T>,
    count: usize,
}
impl<'a, T: Ord> PQAlgo<'a, T> {
    pub fn new(vec: &'a mut Vec<T>) -> Self {
        PQAlgo {
            vec: vec,
            count: 0,
        }
    }
    pub fn insert(&mut self, item: T) {
        self.vec.push(item);
        let len = self.vec.len();
        self.swim(len);
    }
    pub fn del_max(&mut self) -> T {
        let len = self.vec.len() - 1;
        self.vec.swap(1, len);
        let value = self.vec.pop().unwrap();
        self.sink(1);
        value
    }
    fn swim(&mut self, index: usize) {
        let mut move_index = index;
        while move_index > 1 && self.vec[move_index / 2] < self.vec[move_index] {
            self.vec.swap(move_index / 2, move_index);
            move_index = move_index / 2;
        }
    }
    fn sink(&mut self, index: usize) {
        let mut move_index = index;
        while 2 * move_index <= self.vec.len() {
            let mut j = 2 * move_index;
            if j < self.vec.len() && self.vec[j] < self.vec[j + 1] {
                j += 1;
            }
            if move_index >= j {
                break;
            }
            self.vec.swap(move_index, j);
            move_index = j;
        }
    }
    pub fn is_empty(&self) -> bool {
        self.vec.len() > 0
    }
    pub fn size(&self) -> usize {
        self.vec.len()
    }
}
