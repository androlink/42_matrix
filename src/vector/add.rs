use std::ops::{Add, AddAssign};

use super::Vector;

impl<K: std::ops::AddAssign + Copy, const N: usize> Vector<K, N> {
    pub fn add(&mut self, v: &Vector<K, N>) {
        *self += v;
    }
}

impl<K: Add<Output = K> + Default + Copy, const N: usize> Add<Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self + &other
    }
}

impl<K: Add<Output = K> + Default + Copy, const N: usize> Add<&Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self.data
            .iter_mut()
            .zip(&other.data)
            .for_each(|(a, b)| *a = *a + *b);
        self
    }
}

impl<K: AddAssign + Clone + Copy, const N: usize> AddAssign<Vector<K, N>> for Vector<K, N> {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}

impl<K: AddAssign + Clone + Copy, const N: usize> AddAssign<&Vector<K, N>> for Vector<K, N> {
    fn add_assign(&mut self, other: &Self) {
        self.data
            .iter_mut()
            .zip(&other.data)
            .for_each(|(a, b)| *a += *b);
    }
}
