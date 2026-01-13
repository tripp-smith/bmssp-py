use std::cmp::{Ordering, PartialEq, PartialOrd};
use num_traits::Float;

/// Wrapper type to make floats orderable for use in data structures requiring Ord
/// (like BTreeSet, BinaryHeap, etc.)
/// 
/// Treats NaN as greater than all other values (consistent with IEEE 754)
#[derive(Clone, Copy, Debug)]
pub struct OrderedFloat<T>(pub T);

impl<T: Float> PartialEq for OrderedFloat<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Float> Eq for OrderedFloat<T> {}

impl<T: Float> PartialOrd for OrderedFloat<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Float> Ord for OrderedFloat<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or_else(|| {
            // Handle NaN: treat as greater than everything
            if self.0.is_nan() {
                if other.0.is_nan() {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Less
            }
        })
    }
}
