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
    pub fn new(init: T) -> LinkedListStack<T> {
        let node = Rc::new(Node {
            value: Some(init),
            next: None,
        });
        LinkedListStack {
            first: Rc::clone(&node),
            last: node,
            size: 1,
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
        // Always contains at least 1 item
        false
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct LinkedListStackIterator<'a, T: 'a> {
    item: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListStackIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let value: Option<&T> = match self.item {
            Some(ref node_ref) => node_ref.value.as_ref(),
            None => None,
        };

        let next_item: Option<&Node<T>> = match self.item {
            Some(ref node_ref) => match node_ref.next {
                Some(ref next_node_ref) => Some(next_node_ref),
                None => None,
            },
            None => None,
        };
        ::std::mem::replace(&mut self.item, next_item);

        value
    }
}

impl<'a, T> IntoIterator for &'a LinkedListStack<T> {
    type Item = &'a T;
    type IntoIter = LinkedListStackIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListStackIterator {
            item: Some(&self.first),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_behavior() {
        let list = LinkedListStack::new(0);
        assert_eq!(Some(0), list.first.value);
        assert_eq!(None, list.first.next);
        assert!(!list.is_empty());
        assert_eq!(1, list.size());
    }

    #[test]
    fn push_and_pop() {
        let mut list = LinkedListStack::new(0);
        list.push(1);

        // New value is on top of the stack
        assert_eq!(Some(1), list.first.value);
        assert_eq!(2, list.size());

        // Old value is at the bottom
        if let Some(ref node) = list.first.next {
            assert_eq!(Some(0), node.value);
        }

        list.pop();

        // Old value now at the top
        assert_eq!(Some(0), list.first.value);
        assert_eq!(1, list.size());

        list.pop();
        // Pop with 1 value has no effect
        assert_eq!(Some(0), list.first.value);
        assert_eq!(1, list.size());
    }

    #[test]
    fn iterator_implementation() {
        let mut list = LinkedListStack::new(0);
        list.push(1);
        list.push(2);

        // Temporary immutable borrow to check elements
        {
            let elements: Vec<&i32> = list.into_iter().collect();
            assert_eq!(vec![&2, &1, &0], elements);
        }

        list.pop();
        list.pop();

        let elements: Vec<&i32> = list.into_iter().collect();
        assert_eq!(vec![&0], elements);
    }
}
