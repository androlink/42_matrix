use super::Matrix;

impl<K: Copy + std::ops::MulAssign<K>, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn scl(&mut self, a: K) {
        *self *= a;
    }
}
