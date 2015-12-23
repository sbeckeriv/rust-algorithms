pub struct MaxStack<T> {
    max: usize,
    data: Vec<T>,
}

pub struct MaxStackIterator<T> {
    current_pos: usize,
    object: MaxStack<T>,
}

impl<T> MaxStackIterator<T> {
    pub fn new(stack: MaxStack<T>) -> MaxStackIterator<T> {
        MaxStackIterator {
            current_pos: 0,
            object: stack,
        }
    }
}

impl<'a, T> Iterator for &'a MaxStackIterator<T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.object.data.len() - 1 - self.current_pos;
        match self.object.data.get(pos) {
            Some(item) => {
                self.current_pos = self.current_pos + 1;
                Some(item)
            }
            None => None,
        }
    }
}

impl<T> IntoIterator for MaxStack<T> {
    type Item = T;
    type IntoIter = MaxStackIterator<T>;

    fn into_iter(self) -> MaxStackIterator<T> {
        MaxStackIterator::new(self)
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
