
pub struct LinkNode<T> {
    item: T,
    pub next: Option<LinkNode<T>>,
}

impl<T> LinkNode<T> {
    pub fn new(item: T) -> LinkNode<T> {
        LinkNode {
            item: item,
            next: None,
        }
    }
}
