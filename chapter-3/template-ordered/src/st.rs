pub struct Key {
    name: String,
}
pub struct Value {
    value: usize,
}

pub struct Table;
impl Table {
    fn new() -> Self {
        Table
    }
    pub fn put(&mut self, key: Key, value: Value) {}
    pub fn get(&self, key: Key) -> Value {}
    pub fn delete(&mut self, key: Key) -> Value {}
    pub fn contains(&self, key: Key) -> bool {}
    pub fn is_empty(&self) -> bool {}
    pub fn size(&self) -> isize {}
    pub fn keys(&self) -> Vec<Key> {}
    pub fn min(&self) -> Key {}
    pub fn max(&self) -> Key {}
    pub fn floor(&self, key: Key) -> Key {}
    pub fn ceiling(&self, key: Key) -> Key {}
    pub fn rank(&self, key: Key) -> isize {}
    pub fn select(&self, index: isze) -> Key {}
    pub fn delete_min(&mut self) {}
    pub fn delete_max(&mut self) {}
}
