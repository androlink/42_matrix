use std::ops::{Sub, SubAssign};

use super::Matrix;

impl<K: Copy + std::ops::SubAssign<K>, const M: usize, const N: usize> Matrix<K, M, N> {
    /**
     *
     * complexity: O(N*M)
     */
    pub fn sub(&mut self, v: &Matrix<K, M, N>) {
        *self -= *v;
    }
}

impl<K: Sub<Output = K> + Default + Copy, const M: usize, const N: usize> Sub<&Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    type Output = Self;

    /**
     *
     * complexity: O(N*M)
     */
    fn sub(self, other: &Self) -> Self::Output {
        self - *other
    }
}

impl<K: Sub<Output = K> + Default + Copy, const M: usize, const N: usize> Sub<Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    type Output = Self;
    /**
     *
     * complexity: O(N*M)
     */
    fn sub(self, other: Self) -> Self::Output {
        let mut mat = self;
        mat.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| *v1 = v1.sub(*v2));
        mat
    }
}

impl<K: SubAssign + Clone + Copy, const M: usize, const N: usize> SubAssign<&Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    /**
     *
     * complexity: O(N*M)
     */
    fn sub_assign(&mut self, other: &Self) {
        *self -= *other
    }
}

impl<K: SubAssign + Clone + Copy, const M: usize, const N: usize> SubAssign<Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    /**
     *
     * complexity: O(N*M)
     */
    fn sub_assign(&mut self, other: Self) {
        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| v1.sub_assign(*v2));
    }
}
