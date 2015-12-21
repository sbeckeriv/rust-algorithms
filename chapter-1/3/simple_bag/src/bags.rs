pub struct Ziploc<T> {
    data: Vec<T>,
}
// Thoughts
// new with capacity much like vec
// the code example does math. I could limit T to anything that implements math values?
impl<T> Ziploc<T> {
    pub fn new() -> Ziploc<T> {
        Ziploc { data: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> IntoIterator for Ziploc<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
