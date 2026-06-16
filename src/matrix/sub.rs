use super::Matrix;

impl<K: Copy + std::ops::SubAssign<K>, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn sub(&mut self, v: &Matrix<K, M, N>) {
        *self -= *v;
    }
}
