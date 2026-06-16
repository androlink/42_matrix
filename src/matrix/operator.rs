use crate::vector::Vector;

use super::Matrix;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

impl<K: Add<Output = K> + Default + Copy, const M: usize, const N: usize> Add for Matrix<K, M, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut mat = self;
        mat.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| *v1 = v1.add(*v2));
        mat
    }
}

impl<K: AddAssign + Clone + Copy, const M: usize, const N: usize> AddAssign for Matrix<K, M, N> {
    fn add_assign(&mut self, other: Self) {
        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| v1.add_assign(*v2));
    }
}

impl<K: Sub<Output = K> + Default + Copy, const M: usize, const N: usize> Sub for Matrix<K, M, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut mat = self;
        mat.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| *v1 = v1.sub(*v2));
        mat
    }
}

impl<K: SubAssign + Clone + Copy, const M: usize, const N: usize> SubAssign for Matrix<K, M, N> {
    fn sub_assign(&mut self, other: Self) {
        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(v1, v2)| v1.sub_assign(*v2));
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

impl<K: MulAssign + Clone + Copy, const M: usize, const N: usize> MulAssign<K> for Matrix<K, M, N> {
    fn mul_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|v| v.mul_assign(scalar));
    }
}

impl<K: Div<Output = K> + Default + Copy, const M: usize, const N: usize> Div<K>
    for Matrix<K, M, N>
{
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        self.data.map(|v| v.div(scalar)).into()
    }
}

impl<K: DivAssign + Clone + Copy, const M: usize, const N: usize> DivAssign<K> for Matrix<K, M, N> {
    fn div_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|d| d.div_assign(scalar));
    }
}

impl<K: Clone + Copy, const M: usize, const N: usize> Index<usize> for Matrix<K, M, N> {
    type Output = Vector<K, N>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K: Clone + Copy, const M: usize, const N: usize> IndexMut<usize> for Matrix<K, M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Vector<K, N> {
        &mut self.data[index]
    }
}

// impl<K> Matrix::<K> {
// fn mul_vec::<K>(&mut self, vec: Vector::<K>) -> Vector::<K>;
// fn mul_mat::<K>(&mut self, mat: Matrix::<K>) -> Matrix::<K>;
// }

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

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;
    use crate::vector::Vector;
    #[test]
    fn mul_vector() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(u * v, [4., 2.].into());
        let u = Matrix::from([[2., 0.], [0., 2.]]);
        assert_eq!(u * v, [8., 4.].into());
        let u = Matrix::from([[2., -2.], [-2., 2.]]);
        assert_eq!(u * v, [4., -4.].into());
    }
    #[test]
    fn mul_matrix() {
        let u = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u * v, [[1., 0.], [0., 1.]].into());
        let v = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u * v, [[2., 1.], [4., 2.]].into());
        let u = Matrix::from([[3., -5.], [6., 8.]]);
        assert_eq!(u * v, [[-14., -7.], [44., 22.]].into());
    }
}
