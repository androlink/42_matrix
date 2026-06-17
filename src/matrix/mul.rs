use std::ops::{Add, Mul, MulAssign};

use crate::{matrix::Matrix, vector::Vector};

impl<K: Mul<Output = K> + Default + Copy, const M: usize, const N: usize> Mul<&K>
    for Matrix<K, M, N>
{
    type Output = Self;

    fn mul(self, scalar: &K) -> Self::Output {
        self * *scalar
    }
}

impl<K: Mul<Output = K> + Default + Copy, const M: usize, const N: usize> Mul<K>
    for Matrix<K, M, N>
{
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        self.data.map(|v| v.mul(scalar)).into()
    }
}

impl<K: MulAssign + Clone + Copy, const M: usize, const N: usize> MulAssign<&K>
    for Matrix<K, M, N>
{
    fn mul_assign(&mut self, scalar: &K) {
        *self *= *scalar
    }
}

impl<K: MulAssign + Clone + Copy, const M: usize, const N: usize> MulAssign<K> for Matrix<K, M, N> {
    fn mul_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|v| v.mul_assign(scalar));
    }
}

impl<K, const M: usize, const N: usize, const P: usize> Mul<Matrix<K, N, P>> for Matrix<K, M, N>
where
    K: Default + Copy + Mul<Output = K> + Add<Output = K>,
{
    type Output = Matrix<K, M, P>;

    fn mul(self, rhs: Matrix<K, N, P>) -> Self::Output {
        let rhs = rhs.transpose();
        let mut res = Self::Output::default();
        res.iter_mut()
            .zip(*self)
            .for_each(|(v, vec)| *v = rhs * vec);
        res
    }
}

impl<K, const M: usize, const N: usize, const P: usize> Mul<&Matrix<K, N, P>> for Matrix<K, M, N>
where
    K: Default + Copy + Mul<Output = K> + Add<Output = K>,
{
    type Output = Matrix<K, M, P>;

    fn mul(self, rhs: &Matrix<K, N, P>) -> Self::Output {
        self * *rhs
    }
}

impl<K, const M: usize, const N: usize> Mul<&Vector<K, N>> for Matrix<K, M, N>
where
    K: Default + Copy + Mul<Output = K> + Add<Output = K>,
{
    type Output = Vector<K, M>;

    fn mul(self, rhs: &Vector<K, N>) -> Self::Output {
        self * *rhs
    }
}

impl<K, const M: usize, const N: usize> Mul<Vector<K, N>> for Matrix<K, M, N>
where
    K: Default + Copy + Mul<Output = K> + Add<Output = K>,
{
    type Output = Vector<K, M>;

    fn mul(self, rhs: Vector<K, N>) -> Self::Output {
        let mut vec = Self::Output::default();
        vec.iter_mut()
            .zip(*self)
            .for_each(|(v, mat_v)| *v = rhs.dot(mat_v));
        vec
    }
}
