pub struct MaxStack<T> {
    max: usize,
    data: Vec<T>,
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
