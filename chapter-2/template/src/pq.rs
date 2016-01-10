pub struct PQAlgo<'a, T: 'a> {
    vec: &'a mut Vec<String>,
    count: usize,
}
impl<'a, T: Ord> PQAlgo<'a, T> {
    pub fn new(vec: &'a mut Vec<T>) -> Self {
        PQAlgo {
            vec: vec,
            count: 0,
        }
    }
}
