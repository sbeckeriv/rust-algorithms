#[derive(Debug)]
pub struct LinkList<T> {
    head: Option<Box<LinkNode<T>>>,
}

impl<T> LinkList<T> {
    pub fn new() -> LinkList<T> {
        LinkList { head: None }
    }
    pub fn append(&mut self, item: T) {
        let mut head = self.head.unwrap();
        loop {
            match head.next {
                Some(_) => head = head.next.unwrap(),
                None => {}
            }

        }
    }
}
#[derive(Debug, Clone)]
pub struct LinkNode<T> {
    item: T,
    pub next: Option<Box<LinkNode<T>>>,
}

impl<T> LinkNode<T> {
    pub fn new(item: T) -> LinkNode<T> {
        LinkNode {
            item: item,
            next: None,
        }
    }
    pub fn set_next(&mut self, next_node: Box<LinkNode<T>>) {
        self.next = Some(next_node);
    }
}
