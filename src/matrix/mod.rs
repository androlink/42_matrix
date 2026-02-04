mod add;
mod scale;
mod sub;

mod operator;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Matrix<K, const N: usize, const M: usize> {
    data: [[K; N]; M],
}

impl<K: std::cmp::PartialEq, const N: usize, const M: usize> Matrix<K, N, M> {
    pub fn size(&self) -> (usize, usize) {
        (N, M)
    }
}

impl<K: Clone, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K, N, M> {
    fn from(value: [[K; N]; M]) -> Self {
        Self { data: value }
    }
}

impl<K: Default + Copy, const N: usize, const M: usize> Default for Matrix<K, N, M>
{
  fn default() -> Self
  {
    Self {data: [[K::default(); N]; M]}
  }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[8.0, 6.0], [1.0, 6.0]].into();
        m1.add(&m2);
        assert_eq!(m1, res, "add {:?} and {:?}", "lol", "lol");
    }
    #[test]
    fn test_sub() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let m2 = Matrix::from([[7., 4.], [-2., 2.]]);
        let res = [[-6.0, -2.0], [5.0, 2.0]].into();
        m1.sub(&m2);
        assert_eq!(m1, res, "sub {:?} and {:?}", m1, m2);
    }

    #[test]
    fn test_scale() {
        let mut m1 = Matrix::from([[1., 2.], [3., 4.]]);
        let res = [[2.0, 4.0], [6.0, 8.0]].into();
        m1.scl(2.);
        assert_eq!(m1, res, "scale {:?} by {:?}", m1, 2.);
    }
}
