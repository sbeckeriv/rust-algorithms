
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
        self.vec.sort();
    }
    pub fn max(&self) -> &T {
        &self.vec[0]
    }
    pub fn del_max(&mut self) -> T {
        self.vec.pop().unwrap()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.len() > 0
    }
    pub fn size(&self) -> usize {
        self.vec.len()
    }
}
