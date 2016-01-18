use std::collections::LinkedList;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Key(String);
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(usize);
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct KeyVaule(Key, Value);

pub struct Table {
    link_list: LinkedList<KeyVaule>,
}

impl Table {
    fn new() -> Self {
        Table { link_list: LinkedList::new() }
    }
    pub fn put(&mut self, key: Key, value: Value) {
        for node in &mut self.link_list {
            if node.0 == key {
                node.1 = value;
                return;
            }
        }
        let mut new_key = KeyVaule(key, value);
        self.link_list.push_back(new_key);
    }

    pub fn get(&self, key: Key) -> Option<Value> {
        self.link_list
            .iter()
            .filter_map(|&x| {
                if x.0 == key {
                    Some(Value(3))
                } else {
                    None
                }
            })
            .collect()
    }
    pub fn is_empty(&self) -> bool {
        self.link_list.is_empty()
    }
    pub fn size(&self) -> usize {
        self.link_list.len()
    }
    // pub fn delete(&mut self, key: Key) -> Value {}
    // pub fn contains(&self, key: Key) -> bool {}
    // pub fn keys(&self) -> Vec<Key> {}
    //
}
