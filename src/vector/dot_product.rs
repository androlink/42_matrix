use std::ops::{Add, Mul};

use super::Vector;

impl<K, const N: usize> Vector<K, N>
where
    K: Default + Mul<Output = K> + Add<Output = K> + Clone + Copy,
{
    /**
     *
     * time complexity: O(N)
     */
    pub fn dot(&self, v: Self) -> K {
        self.data
            .iter()
            .zip(v.data.iter())
            .fold(K::default(), |acc, (a, b)| acc + *a * *b)
    }
}
