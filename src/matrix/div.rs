use std::ops::{Div, DivAssign};

use crate::matrix::Matrix;

impl<K: Div<Output = K> + Default + Copy, const M: usize, const N: usize> Div<&K>
    for Matrix<K, M, N>
{
    type Output = Self;
    /**
     *
     * complexity: O(NM)
     */
    fn div(self, scalar: &K) -> Self::Output {
        self / *scalar
    }
}
impl<K: Div<Output = K> + Default + Copy, const M: usize, const N: usize> Div<K>
    for Matrix<K, M, N>
{
    type Output = Self;
    /**
     *
     * complexity: O(NM)
     */
    fn div(self, scalar: K) -> Self::Output {
        self.data.map(|v| v.div(scalar)).into()
    }
}

impl<K: DivAssign + Clone + Copy, const M: usize, const N: usize> DivAssign<&K>
    for Matrix<K, M, N>
{
    /**
     *
     * complexity: O(NM)
     */
    fn div_assign(&mut self, scalar: &K) {
        *self /= *scalar
    }
}

impl<K: DivAssign + Clone + Copy, const M: usize, const N: usize> DivAssign<K> for Matrix<K, M, N> {
    /**
     *
     * complexity: O(NM)
     */
    fn div_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|d| d.div_assign(scalar));
    }
}
