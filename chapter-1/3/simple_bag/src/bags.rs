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
// //So this version returns a reference to the thing.
// https://gist.github.com/anonymous/d6eb9a2709e79d69cd43
// mbrubeck: sbeckeriv: Vec<T> has three different standard iterators
// [3:56pm] mbrubeck: sbeckeriv: vec.iter() yields &T, vec.iter_mut() yields &mut T, and vec.into_iter() yields T
// [3:58pm] mbrubeck: And there are three different IntoIterator implementations, corresponding to each of thes.
// impl<T> IntoIterator for Ziploc<T> {
// type Item = T;
// type IntoIter = ::std::vec::IntoIter<T>;
//
// fn into_iter(self) -> Self::IntoIter {
// self.data.into_iter()
// }
// }
//
impl<'a, T> IntoIterator for &'a Ziploc<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
