use std::rc::Rc;

#[derive(Debug)]
pub struct LinkedListStack<T> {
    first: Rc<Node<T>>,
    last: Rc<Node<T>>,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: Option<T>,
    next: Option<Rc<Node<T>>>,
}

impl<T> LinkedListStack<T> {
    pub fn new() -> LinkedListStack<T> {
        let node = Rc::new(Node {
            value: None,
            next: None,
        });
        LinkedListStack {
            first: Rc::clone(&node),
            last: node,
        }
    }

    pub fn push(&mut self, item: T) {
        let new_first: Rc<Node<T>> = Rc::new(Node {
            value: Some(item),
            next: Some(Rc::clone(&self.first)),
        });
        self.first = new_first;
        self.size += 1;
    }

    pub fn pop(&mut self) {
        if !Rc::ptr_eq(&self.first, &self.last) {
            let new_first: Rc<Node<T>> = Rc::get_mut(&mut self.first).unwrap().next.take().unwrap();
            self.first = new_first;
            self.size -= 1;
        }
    }

    pub fn is_empty(&self) -> bool {
        Rc::ptr_eq(&self.first, &self.last) && Rc::get(&self.first).unwrap().value.is_none();
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_behavior() {
        let list: LinkedListStack<i32> = LinkedListStack::new();
        let first: &Node<i32> = Rc::get(&list.first).unwrap();
        assert_eq!(None, first.value);
        assert_eq!(None, first.next);
        assert!(list.is_empty());
        assert_eq!(1, list.size());
    }
}
