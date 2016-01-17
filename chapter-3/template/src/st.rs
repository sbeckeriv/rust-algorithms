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
}
