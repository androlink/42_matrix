use std::ops::Mul;

use super::*;
use crate::vector::Vector;

// impl<K> Matrix::<K> {
// fn mul_vec::<K>(&mut self, vec: Vector::<K>) -> Vector::<K>;
// fn mul_mat::<K>(&mut self, mat: Matrix::<K>) -> Matrix::<K>;
// }

impl<K, const N: usize, const M: usize, const P: usize> Mul<Matrix<K, P, N>> for Matrix<K, N, M>
where
    K: Default,
    K: Copy,
{
    type Output = Matrix<K, P, M>;

    fn mul(self, mat: Matrix<K, P, N>) -> Self::Output {
        let mut res = Self::Output::default();
        for i in 0..P {
            for j in 0..M {}
        }
        todo!()
    }
}

impl<K, const N: usize, const M: usize> Mul<Vector<K, N>> for Matrix<K, N, M>
where
    K: Default,
    K: Copy,
{
    type Output = Vector<K, M>;

    fn mul(self, vec: Vector<K, N>) -> Self::Output {
        todo!()
    }
}
