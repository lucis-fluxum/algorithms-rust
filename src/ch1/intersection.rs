use std::collections::HashSet;
use std::hash::Hash;

// Note: order is not preserved
pub fn intersection<'a, T>(first: &'a [T], second: &'a [T]) -> Vec<&'a T>
where
    T: Hash + Eq,
{
    // Allocate worst-case sizes for these
    let mut existences = HashSet::with_capacity(first.len().max(second.len()));
    let mut intersection = HashSet::with_capacity(first.len().min(second.len()));

    for item in first {
        existences.insert(item);
    }

    for item in second {
        if existences.contains(item) {
            intersection.insert(item);
        }
    }

    intersection.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_behavior() {
        let first = [1, 2, 3, 4, 5, 6, 7];
        let second = [2, 4, 6, 8];
        check_match(&vec![&2, &4, &6], &intersection(&first, &second));

        let first = ["a", "b", "c", "d"];
        let second = ["a", "c", "e"];
        check_match(&vec![&"a", &"c"], &intersection(&first, &second));
    }

    fn check_match<T>(first: &Vec<T>, second: &Vec<T>)
    where
        T: Eq,
    {
        for element in first {
            assert!(second.contains(element));
        }

        for element in second {
            assert!(first.contains(element));
        }
    }
}
