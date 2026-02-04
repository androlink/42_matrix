use super::Matrix;

use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;
use std::ops::AddAssign;
use std::ops::SubAssign;

impl<K: Add<Output= K> + Default + Copy, const N: usize, const M: usize> Add for Matrix<K, N, M>
{
  type Output = Self;

  fn add(self, other: Self) -> Self::Output
  {
    let mut v: Matrix<K, N, M> = Matrix::from([[K::default(); N];M]);
    for i in 0..M{
      for j in 0..N{
        v.data[i][j] = self.data[i][j] + other.data[i][j];
      }
    }
    v
  }

}

impl<K: Sub<Output= K> + Default + Copy, const N: usize, const M: usize> Sub for Matrix<K, N, M>
{
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output
  {
    let mut v: Matrix<K, N, M> = Matrix::from([[K::default(); N];M]);
    for i in 0..M{
      for j in 0..N{
        v.data[i][j] = self.data[i][j] - other.data[i][j];
      }
    }
    v
  }

}

impl<K: AddAssign + Clone + Copy, const N: usize, const M: usize> AddAssign for Matrix<K, N, M>
{

  fn add_assign(&mut self, other: Self)
  {
    for i in 0..M{
      for j in 0..N{
        self.data[i][j] += other.data[i][j];
      }
    }
    ()
  }
}

impl<K: SubAssign + Clone + Copy, const N: usize, const M: usize> SubAssign for Matrix<K, N, M>
{

  fn sub_assign(&mut self, other: Self)
  {
    for i in 0..M{
      for j in 0..N{
        self.data[i][j] -= other.data[i][j];
      }
    }
    ()
  }
}

impl<K: SubAssign + Clone + Copy, const N: usize, const M: usize> Index<usize> for Matrix<K, N, M>
{
  type Output = [K; N];
  fn index(&self, index: usize) -> &Self::Output {
      &self.data[index]
  }
}

impl<K: SubAssign + Clone + Copy, const N: usize, const M: usize> IndexMut<usize> for Matrix<K, N, M>
{
  fn index_mut(&mut self, index: usize) -> &mut [K; N] {
      &mut self.data[index]
  }
}

#[cfg(test)]
mod test
{
  use super::*;
  #[test]
  fn add_assign_test()
  {
    let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
    let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
    let res = [[8.0, 6.0], [1.0, 6.0]].into();
    m1 += m2;
    assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
  }
  #[test]
  fn sub_assign_test()
  {
    let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
    let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
    let res = [[-6.0, -2.0], [5.0, 2.0]].into();
    m1 -= m2;
    assert_eq!(m1, res, "result {:?}, assert {:?}", m1, res);
  }
  #[test]
  fn test_add() {
    let m1 = Matrix::from([[1., 2.], [3., 4.]]);
    let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
    let res = [[8.0, 6.0], [1.0, 6.0]].into();
    let result = m1 + m2;
    assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
  }
  #[test]
  fn test_sub() {
    let m1 = Matrix::from([[1., 2.], [3., 4.]]);
    let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
    let res = [[-6.0, -2.0], [5.0, 2.0]].into();
    let result = m1 - m2;
    assert_eq!(result, res, "result {:?}, assert {:?}", result, res);
  }
}