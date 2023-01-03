//! Small library for generating distinct permutations of a vector
use itertools::Itertools;
use std::{collections::HashMap, hash::Hash};

struct Counts<T>
where
    T: Eq + Hash + Clone,
{
    counts: HashMap<T, usize>,
}

/// Small abstraction of counts, where we only care about the remaining nonzero keys and whether any values are left
impl<T> Counts<T>
where
    T: Eq + Hash + Clone,
{
    pub fn from(counts: HashMap<T, usize>) -> Self {
        for v in counts.values() {
            assert!(*v > 0);
        }
        Self { counts }
    }

    pub fn keys(&self) -> Vec<T> {
        self.counts.keys().cloned().collect_vec()
    }

    pub fn add(&mut self, k: T) {
        let entry = self.counts.entry(k).or_default();
        *entry += 1;
    }

    pub fn remove(&mut self, k: &T) {
        let remove = {
            let entry = self.counts.entry(k.clone()).or_default();
            *entry -= 1;
            *entry == 0
        };
        if remove {
            self.counts.remove(k);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}

/// Mutating recursive implementation of generating permutations starting with `head` and
/// ending with elements from `counts`.
fn distinct_permutations_with<T>(head: &mut Vec<T>, counts: &mut Counts<T>) -> Vec<Vec<T>>
where
    T: Eq + Hash + Clone + Ord,
{
    let mut result = vec![];
    let mut keys = counts.keys();
    keys.sort();
    for value in keys {
        head.push(value.clone());
        counts.remove(&value);
        if counts.is_empty() {
            result.push(head.clone())
        } else {
            result.append(&mut distinct_permutations_with(head, counts))
        }
        head.pop();
        counts.add(value);
    }
    result
}

/// Returns the permutations of the input vector that are distinct with
/// respect to `Eq` on `T`, lexicographically sorted.
///
/// The runtime is proportional to the size of the output generated,
/// not the total number of permutations ignoring equality.
///
/// For inputs with unique elements, this function generates all permutations.
///
///# Examples
/// ```rust
/// # use distinct_permutations::distinct_permutations;
/// assert_eq!(distinct_permutations(vec![0, 0, 1]),
///     vec![
///         vec![0, 0, 1],
///         vec![0, 1, 0],
///         vec![1, 0, 0]
/// ]);
/// ```
pub fn distinct_permutations<T>(input: Vec<T>) -> Vec<Vec<T>>
where
    T: Eq + Hash + Clone + Ord,
{
    let total_len = input.len();
    let mut counts = Counts::from(input.into_iter().counts());
    let mut head = Vec::with_capacity(total_len);
    distinct_permutations_with(&mut head, &mut counts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_empty() {
        assert_eq!(distinct_permutations::<u64>(vec![]), Vec::<Vec<u64>>::new());
    }

    #[test]
    fn test_small_unique() {
        assert_eq!(distinct_permutations(vec![0]), vec![vec![0]]);
        assert_eq!(
            distinct_permutations(vec![0, 1]),
            vec![vec![0, 1], vec![1, 0]]
        );
        assert_eq!(
            distinct_permutations(vec![0, 1, 2]),
            vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0]
            ]
        );
    }

    #[test]
    fn test_small_non_unique() {
        assert_eq!(
            distinct_permutations(vec![0, 0, 1, 1]),
            vec![
                vec![0, 0, 1, 1],
                vec![0, 1, 0, 1],
                vec![0, 1, 1, 0],
                vec![1, 0, 0, 1],
                vec![1, 0, 1, 0],
                vec![1, 1, 0, 0]
            ]
        );
    }

    #[test]
    fn test_input_order_does_not_matter() {
        assert_eq!(
            distinct_permutations(vec![0, 0, 1, 1]),
            distinct_permutations(vec![0, 1, 0, 1]),
        );
    }

    #[test]
    fn test_longer_non_unique() {
        // it's only practical to sample
        assert!(
            distinct_permutations(vec![0, 0, 0, 1, 1, 1, 1]).contains(&vec![1, 1, 0, 0, 1, 0, 1])
        );
        assert!(
            distinct_permutations(vec![0, 0, 0, 1, 1, 1, 2]).contains(&vec![0, 1, 0, 2, 1, 0, 1])
        );
    }

    /// Output be identical to regular permutations for unique elements
    #[test]
    fn test_larger_unique() {
        let mut generated_by_library = vec![0, 1, 2, 3, 4, 5]
            .into_iter()
            .permutations(6)
            .collect_vec();
        generated_by_library.sort();
        assert_eq!(
            distinct_permutations(vec![0, 1, 2, 3, 4, 5]),
            generated_by_library
        )
    }
}
