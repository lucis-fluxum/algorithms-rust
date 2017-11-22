use std::slice;

pub struct Bag<T>(Vec<T>);

impl<T> Bag<T> {
    pub fn new() -> Bag<T> {
        Bag(Vec::new())
    }

    pub fn add(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_behavior() {
        let mut bag: Bag<String> = Bag::new();

        assert_eq!(0, bag.size());
        assert!(bag.is_empty());

        bag.add(String::from("string1"));
        bag.add(String::from("string2"));
        bag.add(String::from("string3"));

        for item in &bag {
            assert_eq!(7, item.len());
        }

        assert_eq!(3, bag.size());
        assert!(!bag.is_empty());
    }
}
