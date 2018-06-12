use std::collections::HashMap;
use std::hash::Hash;

pub fn intersection<'a, T>(first: &'a [T], second: &'a [T]) -> Vec<&'a T>
where
    T: Hash + Eq,
{
    // Allocate worst-case sizes for these
    let mut existences = HashMap::with_capacity(first.len().max(second.len()));
    let mut intersection = Vec::with_capacity(first.len().min(second.len()));

    for item in first {
        if !existences.contains_key(item) {
            existences.insert(item, true);
        }
    }

    for item in second {
        if existences.contains_key(item) {
            intersection.push(item);
        }
    }

    intersection
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_behavior() {
        let first = [1, 2, 3, 4, 5, 6, 7];
        let second = [2, 4, 6, 8];
        assert_eq!(vec![&2, &4, &6], intersection(&first, &second));

        let first = ["a", "b", "c", "d"];
        let second = ["a", "c", "e"];
        assert_eq!(vec![&"a", &"c"], intersection(&first, &second));
    }
}
