use super::Matrix;

impl<K: Copy + std::ops::SubAssign<K>, const N: usize, const M: usize> Matrix<K, N, M> {
    pub fn sub(&mut self, v: &Matrix<K, N, M>) {
        *self -= *v;
    }
}
