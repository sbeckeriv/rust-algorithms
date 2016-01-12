#[derive(Debug)]
pub struct PQAlgo<'a, T: 'a> {
    vec: &'a mut Vec<T>,
    count: usize,
}

impl<'a, T: Ord> PQAlgoArray<'a, T> for PQAlgo<'a, T> {
    fn new(vec: &'a mut Vec<T>) -> PQAlgo<'a, T> {
        PQAlgo {
            vec: vec,
            count: 0,
        }
    }
}

pub trait PQAlgoArray<'a, T: Ord> {
    fn new(vec: &'a mut Vec<T>) -> Self;
    fn insert(&'a mut self, item: T) {
        self.vec.push(item);
        self.vec.sort();
    }
    fn max(&self) -> &T {
        &self.vec[0]
    }
    fn del_max(&mut self) -> T {
        self.vec.pop().unwrap()
    }
    fn is_empty(&self) -> bool {
        self.vec.len() > 0
    }
    fn size(&self) -> usize {
        self.vec.len()
    }
}
