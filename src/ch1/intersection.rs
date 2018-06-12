use std::collections::HashSet;
use std::hash::Hash;

// Arrays need not be sorted. Order is not preserved.
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

// We can also use standard library features.
pub fn alternate_intersection<'a, T>(first: &'a [T], second: &'a [T]) -> Vec<&'a T>
where
    T: Hash + Eq,
{
    let first: HashSet<&T> = first.iter().collect();
    let second: HashSet<&T> = second.iter().collect();
    first.intersection(&second).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_behavior() {
        let first = [1, 2, 2, 3, 4, 5, 1, 6, 5, 7];
        let second = [2, 4, 4, 2, 8, 6, 8];
        check_match(&vec![&2, &4, &6], &intersection(&first, &second));
        check_match(&vec![&2, &4, &6], &alternate_intersection(&first, &second));

        let first = ["z", "a", "c", "d", "b", "f"];
        let second = ["a", "e", "c", "e", "a"];
        check_match(&vec![&"a", &"c"], &intersection(&first, &second));
        check_match(&vec![&"a", &"c"], &alternate_intersection(&first, &second));
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
