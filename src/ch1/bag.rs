pub struct Bag<T> {
    size: i32,
    items: Vec<T>,
}

impl<T> Bag<T> {
    pub fn new() -> Bag<T> {
        Bag {
            size: 0,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_behavior() {
        let mut b: Bag<String> = Bag::new();

        assert_eq!(0, b.size());
        assert!(b.is_empty());

        b.add(String::from("string"));

        assert_eq!(1, b.size());
        assert!(!b.is_empty());
    }
}
