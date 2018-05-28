use std::rc::Rc;

#[derive(Debug)]
pub struct LinkedListStack<T> {
    first: Rc<Node<T>>,
    last: Rc<Node<T>>,
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: Option<T>,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedListStack<T> {
    pub fn new() -> LinkedListStack<T> {
        let node = Rc::new(Node {
            value: None,
            next: None,
        });
        LinkedListStack {
            first: node.clone(),
            last: node,
        }
    }

    pub fn push(item: T) {}

    pub fn pop() {}

    pub fn is_empty() {}

    pub fn size() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn basic_behavior() {
        let list: LinkedListStack<i32> = LinkedListStack::new();
        let first: &Node<i32> = list.first.borrow();
        assert_eq!(None, first.value);
        assert_eq!(None, first.next);
    }
}
