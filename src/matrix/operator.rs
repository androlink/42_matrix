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

impl<K: Add<Output = K> + Default + Copy, const N: usize, const M: usize> Add for Matrix<K, N, M> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut data = self.data;
        data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(row_a, row_b)| {
                row_a
                    .iter_mut()
                    .zip(row_b.iter())
                    .for_each(|(value_a, value_b)| *value_a = *value_a + *value_b)
            });
        Matrix { data }
    }
}

impl<K: AddAssign + Clone + Copy, const N: usize, const M: usize> AddAssign for Matrix<K, N, M> {
    fn add_assign(&mut self, other: Self) {
        self.data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(row_a, row_b)| {
                row_a
                    .iter_mut()
                    .zip(row_b.iter())
                    .for_each(|(value_a, value_b)| *value_a += *value_b)
            });
    }
}

impl<K: Sub<Output = K> + Default + Copy, const N: usize, const M: usize> Sub for Matrix<K, N, M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut data = self.data;
        data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(row_a, row_b)| {
                row_a
                    .iter_mut()
                    .zip(row_b.iter())
                    .for_each(|(value_a, value_b)| *value_a = *value_a - *value_b)
            });
        Matrix { data }
    }
}

impl<K: SubAssign + Clone + Copy, const N: usize, const M: usize> SubAssign for Matrix<K, N, M> {
    fn sub_assign(&mut self, other: Self) {
        self.data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(row_a, row_b)| {
                row_a
                    .iter_mut()
                    .zip(row_b.iter())
                    .for_each(|(value_a, value_b)| *value_a -= *value_b)
            });
    }
}

impl<K: Mul<Output = K> + Default + Copy, const N: usize, const M: usize> Mul<K>
    for Matrix<K, N, M>
{
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        let mut data = self.data;
        data.iter_mut()
            .for_each(|d| d.iter_mut().for_each(|d| *d = *d * scalar));
        Matrix { data }
    }
}

impl<K: MulAssign + Clone + Copy, const N: usize, const M: usize> MulAssign<K> for Matrix<K, N, M> {
    fn mul_assign(&mut self, scalar: K) {
        self.data
            .iter_mut()
            .for_each(|d| d.iter_mut().for_each(|d| *d *= scalar));
    }
}

impl<K: Div<Output = K> + Default + Copy, const N: usize, const M: usize> Div<K>
    for Matrix<K, N, M>
{
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        let mut data = self.data;
        data.iter_mut()
            .for_each(|d| d.iter_mut().for_each(|d| *d = *d / scalar));
        Matrix { data }
    }
}

impl<K: DivAssign + Clone + Copy, const N: usize, const M: usize> DivAssign<K> for Matrix<K, N, M> {
    fn div_assign(&mut self, scalar: K) {
        self.data
            .iter_mut()
            .for_each(|d| d.iter_mut().for_each(|d| *d /= scalar));
    }
}

impl<K: Clone + Copy, const N: usize, const M: usize> Index<usize> for Matrix<K, N, M> {
    type Output = [K; N];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K: Clone + Copy, const N: usize, const M: usize> IndexMut<usize> for Matrix<K, N, M> {
    fn index_mut(&mut self, index: usize) -> &mut [K; N] {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_assign_test() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[8.0, 6.0], [1.0, 6.0]].into();
        m1 += m2;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn add_test() {
        let m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[8.0, 6.0], [1.0, 6.0]].into();
        let result = m1 + m2;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }

    #[test]
    fn sub_assign_test() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[-6.0, -2.0], [5.0, 2.0]].into();
        m1 -= m2;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn sub_test() {
        let m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[-6.0, -2.0], [5.0, 2.0]].into();
        let result = m1 - m2;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }

    #[test]
    fn mul_assign_test() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let scalar = 2.;
        let res = [[2., 4.], [6., 8.]].into();
        m1 *= scalar;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn mul_test() {
        let m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let scalar = 2.;
        let res = [[2., 4.], [6., 8.]].into();
        let result = m1 * scalar;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }

    #[test]
    fn div_assign_test() {
        let mut m1 = Matrix::from([[2., 4.], [6., 8.]]);
        let scalar = 2.;
        let res = [[1., 2.], [3., 4.]].into();
        m1 /= scalar;
        assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
    }

    #[test]
    fn div_test() {
        let m1 = Matrix::from([[2., 4.], [6., 8.]]);
        let scalar = 2.;
        let res = [[1., 2.], [3., 4.]].into();
        let result = m1 / scalar;
        assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
    }
}
