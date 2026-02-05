use crate::matrix::Matrix;

impl<K: Copy + std::ops::MulAssign<K>, const N: usize, const M: usize> Matrix<K, N, M> {
    pub fn scl(&mut self, a: K) {
        *self *= a;
    }
}
