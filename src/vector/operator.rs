use super::Vector;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;
use std::ops::AddAssign;
use std::ops::SubAssign;

impl<K: Add<Output= K> + Default + Copy, const N: usize> Add for Vector<K, N>
{
  type Output = Self;

  fn add(self, other: Self) -> Self::Output
  {
    let mut v: Vector<K, N> = Vector::from([K::default(); N]);
    for i in 0..N{
      v.data[i] = self.data[i] + other.data[i];
    }
    v
  }

}

impl<K: Sub<Output= K> + Default + Copy, const N: usize> Sub for Vector<K, N>
{
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output
  {
    let mut v: Vector<K, N> = Vector::from([K::default(); N]);
    for i in 0..N{
      v.data[i] = self.data[i] - other.data[i];
    }
    v
  }

}

impl<K: AddAssign + Clone + Copy, const N: usize> AddAssign for Vector<K, N>
{

  fn add_assign(&mut self, other: Self)
  {
    for i in 0..N{
      self.data[i] += other.data[i];
    }
    ()
  }
}

impl<K: SubAssign + Clone + Copy, const N: usize> SubAssign for Vector<K, N>
{

  fn sub_assign(&mut self, other: Self)
  {
    for i in 0..N{
      self.data[i] -= other.data[i];
    }
    ()
  }
}

impl<K: SubAssign + Clone + Copy, const N: usize> Index<usize> for Vector<K, N>
{
  type Output = K;
  fn index(&self, index: usize) -> &Self::Output {
      &self.data[index]
  }
}

impl<K: SubAssign + Clone + Copy, const N: usize> IndexMut<usize> for Vector<K, N>
{
  fn index_mut(&mut self, index: usize) -> &mut K {
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
    let mut v1 = Vector::from([2., 3.]);
    let v2 = Vector::from([5., 7.]);
    let res = Vector::from([7., 10.]);
    v1 += v2;
    assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
  }
  #[test]
  fn sub_assign_test()
  {
    let mut v1 = Vector::from([2., 3.]);
    let v2 = Vector::from([5., 7.]);
    let res = Vector::from([-3., -4.]);
    v1 -= v2;
    assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
  }
  #[test]
  fn sub_test()
  {
    let v1 = Vector::from([2., 3.]);
    let v2 = Vector::from([5., 7.]);
    let res = Vector::from([-3., -4.]);
    let result = v1 - v2;
    assert_eq!(result, res, "add {:?} and {:?}", v1, res);
  }
  #[test]
  fn add_test()
  {
    let v1 = Vector::from([2., 3.]);
    let v2 = Vector::from([5., 7.]);
    let res = Vector::from([7., 10.]);
    let result = v1 + v2;
    assert_eq!(result, res, "add {:?} and {:?}", v1, res);
  }
}