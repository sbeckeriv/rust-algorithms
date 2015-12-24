pub struct MaxStack<T> {
    max: usize,
    data: Vec<T>,
}

pub struct MaxStackIterator<'a, T: 'a> {
    current_pos: usize,
    object: &'a MaxStack<T>,
}

impl<'a, T> Iterator for MaxStackIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.object.data.len() as isize - 1 - self.current_pos as isize;
        if pos >= 0 {
            match self.object.data.get(pos as usize) {
                Some(item) => {
                    self.current_pos = self.current_pos + 1;
                    Some(item)
                }
                None => None,
            }
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a MaxStack<T> {
    type Item = &'a T;
    type IntoIter = MaxStackIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MaxStackIterator {
            current_pos: 0,
            object: self,
        }

    }
}

impl<T> MaxStack<T> {
    pub fn new(max: usize) -> MaxStack<T> {
        MaxStack {
            max: max,
            data: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.data.len() < self.max {
            self.data.push(item);
            Ok(())
        } else {
            Err("Stack is full")
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}
