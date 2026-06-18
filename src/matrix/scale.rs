use super::Matrix;

impl<K: Copy + std::ops::MulAssign<K>, const M: usize, const N: usize> Matrix<K, M, N> {
    /**
     *
     * complexity: O(N*M)
     */
    pub fn scl(&mut self, a: K) {
        *self *= a;
    }
}
