mod link;
fn main() {
    let mut list = link::LinkList::new();
    list.append(3);
    println!("{:?}", list);

}
