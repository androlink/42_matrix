use super::Matrix;

impl<K: Copy + std::ops::AddAssign<K>, const N: usize, const M: usize> Matrix<K, N, M> {
    pub fn add(&mut self, v: &Matrix<K, N, M>) {
        *self += *v;
    }
}
