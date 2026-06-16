use super::Matrix;

impl<K: Copy + std::ops::AddAssign<K>, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn add(&mut self, v: &Matrix<K, M, N>) {
        *self += *v;
    }
}
