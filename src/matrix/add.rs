use std::ops::{Add, AddAssign};

use super::Matrix;

impl<K: Copy + std::ops::AddAssign<K>, const M: usize, const N: usize> Matrix<K, M, N> {
    /**
     *
     * time complexity: O(N*M)
     */
    pub fn add(&mut self, v: &Matrix<K, M, N>) {
        *self += *v;
    }
}

impl<K: Add<Output = K> + Default + Copy, const M: usize, const N: usize> Add<&Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    type Output = Self;
    /**
     *
     * time complexity: O(N*M)
     */
    fn add(self, other: &Self) -> Self::Output {
        self + *other
    }
}

impl<K: Add<Output = K> + Default + Copy, const M: usize, const N: usize> Add<Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    type Output = Self;
    /**
     *
     * time complexity: O(N*M)
     */
    fn add(self, other: Self) -> Self::Output {
        let mut mat = self;
        mat.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| *v1 = v1.add(*v2));
        mat
    }
}

impl<K: AddAssign + Clone + Copy, const M: usize, const N: usize> AddAssign<&Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    /**
     *
     * time complexity: O(N*M)
     */
    fn add_assign(&mut self, other: &Self) {
        *self += *other
    }
}

impl<K: AddAssign + Clone + Copy, const M: usize, const N: usize> AddAssign<Matrix<K, M, N>>
    for Matrix<K, M, N>
{
    /**
     *
     * time complexity: O(N*M)
     */
    fn add_assign(&mut self, other: Self) {
        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| v1.add_assign(*v2));
    }
}
