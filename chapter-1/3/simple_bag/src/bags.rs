pub struct Ziploc<'a, T: 'a> {
    data: Vec<&'a T>,
}
// Thoughts
// new with capacity much like vec
// the code example does math. I could limit T to anything that implements math values?
impl<'a, T> Ziploc<'a, T> {
    pub fn new() -> Ziploc<'a, T> {
        Ziploc { data: Vec::new() }
    }

    pub fn add(&mut self, item: &'a T) {
        self.data.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}
// impl<T> IntoIterator for Ziploc<T> {
// type Item = T;
// type IntoIter = ::std::vec::IntoIter<T>;
//
// fn into_iter(self) -> Self::IntoIter {
// self.data.into_iter()
// }
// }
//
impl<'a, T> Iterator for Ziploc<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.iter().next() {
            Some(n) => Some(n),
            None => None,
        }
    }
}
