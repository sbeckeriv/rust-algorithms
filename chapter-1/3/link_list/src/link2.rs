#[derive(Debug)]
enum Links<T> {
    Link(T, Box<Links<T>>),
    End,
}
#[derive(Debug)]
pub struct LinkList<T> {
    head: Links<T>,
}

impl<T> LinkList<T> {
    pub fn new(item: T) -> LinkList<T> {
        LinkList { head: Links::Link(item, Box::new(Links::End)) }
    }
    pub fn append(&mut self, item: T) {
        let mut top = &mut self.head;
        let mut next_link;
        loop {
            match top {
                &mut Links::Link(_, ref mut next) => {
                    next_link = **next;
                    match next_link {
                        Links::Link(_, _) => top = next_link,
                        Links::End => {
                            // new
                        }
                    }
                }
                &mut Links::End => break,
            }
        }
    }
}
