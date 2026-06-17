use std::ops::{Sub, SubAssign};

use super::Vector;

impl<K: Copy + std::ops::SubAssign<K>, const N: usize> Vector<K, N> {
    pub fn sub(&mut self, v: &Vector<K, N>) {
        *self -= v
    }
}

impl<K: Sub<Output = K> + Default + Copy, const N: usize> Sub<Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self - &other
    }
}

impl<K: Sub<Output = K> + Default + Copy, const N: usize> Sub<&Vector<K, N>> for Vector<K, N> {
    type Output = Self;

    fn sub(self, other: &Self) -> Self::Output {
        let mut data = self.data;
        data.iter_mut()
            .zip(&other.data)
            .for_each(|(a, b)| *a = *a - *b);
        Vector { data }
    }
}

impl<K: SubAssign + Clone + Copy, const N: usize> SubAssign<Vector<K, N>> for Vector<K, N> {
    fn sub_assign(&mut self, other: Self) {
        *self -= &other
    }
}

impl<K: SubAssign + Clone + Copy, const N: usize> SubAssign<&Vector<K, N>> for Vector<K, N> {
    fn sub_assign(&mut self, other: &Self) {
        self.data
            .iter_mut()
            .zip(&other.data)
            .for_each(|(a, b)| *a -= *b);
    }
}
