use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::vector::Vector;

mod add;
mod scale;
mod sub;

mod mul;

mod operator;
mod trace;
mod transpose;

pub type Mat2 = Matrix<f32, 2, 2>;
pub type Mat3 = Matrix<f32, 3, 3>;
pub type Mat4 = Matrix<f32, 4, 4>;

/**
 * matrix in column major order as array of Vector<K, N>
 */
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix<K, const M: usize, const N: usize> {
    data: [Vector<K, N>; M],
}

impl<K, const M: usize, const N: usize> Display for Matrix<K, N, M>
where
    K: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl<K: std::cmp::PartialEq, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn size(&self) -> (usize, usize) {
        (N, M)
    }
}

impl<K: Clone, const M: usize, const N: usize> From<[[K; N]; M]> for Matrix<K, M, N> {
    fn from(value: [[K; N]; M]) -> Self {
        Self {
            data: value.map(|v| v.into()),
        }
    }
}

impl<K: Clone, const M: usize, const N: usize> From<[Vector<K, N>; M]> for Matrix<K, M, N> {
    fn from(value: [Vector<K, N>; M]) -> Self {
        Self { data: value }
    }
}

impl<K: Default + Copy, const M: usize, const N: usize> Default for Matrix<K, M, N> {
    fn default() -> Self {
        Self {
            data: [Vector::<K, N>::default(); M],
        }
    }
}

impl<K, const M: usize, const N: usize> Deref for Matrix<K, M, N> {
    type Target = [Vector<K, N>; M];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<K, const M: usize, const N: usize> DerefMut for Matrix<K, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
