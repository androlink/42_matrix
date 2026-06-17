use std::ops::{Mul, MulAssign};

use crate::vector::Vector;

impl<K: Mul<Output = K> + Copy, const N: usize> Mul<K> for Vector<K, N> {
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        self.map(|v| v * scalar).into()
    }
}

impl<K: Mul<Output = K> + Copy, const N: usize> Mul<&K> for Vector<K, N> {
    type Output = Self;

    fn mul(self, scalar: &K) -> Self::Output {
        self * *scalar
    }
}

impl<K, const N: usize> MulAssign<K> for Vector<K, N>
where
    K: MulAssign + Clone + Copy,
{
    fn mul_assign(&mut self, scalar: K) {
        self.iter_mut().for_each(|d| *d *= scalar);
    }
}

impl<K, const N: usize> MulAssign<&K> for Vector<K, N>
where
    K: MulAssign + Clone + Copy,
{
    fn mul_assign(&mut self, scalar: &K) {
        *self *= *scalar
    }
}
