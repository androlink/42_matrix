use std::ops::{Div, DivAssign};

use crate::vector::Vector;

impl<K: Div<Output = K> + Default + Copy, const N: usize> Div<K> for Vector<K, N> {
    type Output = Self;
    /**
     *
     * time complexity: O(N)
     */
    fn div(self, scalar: K) -> Self::Output {
        self.map(|v| v / scalar).into()
    }
}

impl<K: Div<Output = K> + Default + Copy, const N: usize> Div<&K> for Vector<K, N> {
    type Output = Self;
    /**
     *
     * time complexity: O(N)
     */
    fn div(self, scalar: &K) -> Self::Output {
        self / *scalar
    }
}

impl<K, const N: usize> DivAssign<K> for Vector<K, N>
where
    K: DivAssign + Clone + Copy,
{
    /**
     *
     * time complexity: O(N)
     */
    fn div_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|d| *d /= scalar);
    }
}

impl<K, const N: usize> DivAssign<&K> for Vector<K, N>
where
    K: DivAssign + Clone + Copy,
{
    /**
     *
     * time complexity: O(N)
     */
    fn div_assign(&mut self, scalar: &K) {
        *self /= *scalar
    }
}
