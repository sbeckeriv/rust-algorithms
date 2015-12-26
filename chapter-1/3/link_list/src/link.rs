#[derive(Debug)]
pub struct LinkList<T> {
    head: Option<Box<LinkNode<T>>>,
}

impl<T> LinkList<T> {
    pub fn new() -> LinkList<T> {
        LinkList { head: None }
    }
    pub fn append(&mut self, item: T) {
        if self.head.is_some() {
            let mut current = self.head.as_ref();
            loop {
                // 5 | 4 | none
                match current.unwrap().next {
                    Some(_) => current = current.unwrap().next.as_ref(),
                    None => {
                        let mut new_node = LinkNode::new(item);
                        let mut x = current.as_mut().unwrap();
                        x.set_next(Box::new(new_node));
                    }
                }
            }
        } else {
            self.head = Some(Box::new(LinkNode::new(item)));
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
