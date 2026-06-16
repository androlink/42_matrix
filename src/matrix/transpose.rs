use crate::matrix::Matrix;

impl<K: Default + Copy, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn transpose(&self) -> Matrix<K, N, M> {
        let mut m: Matrix<K, N, M> = [[K::default(); M]; N].into();
        for i in 0..M {
            for j in 0..N {
                m[i][j] = self[j][i];
            }
        }
        m
    }
}
