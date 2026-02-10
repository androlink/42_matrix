use super::Vector;
use std::ops::*;

impl<K: Add<Output = K> + Default + Copy, const N: usize> Add for Vector<K, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut data = self.data;
        data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a = *a + *b);
        Vector { data }
    }
}

impl<K: AddAssign + Clone + Copy, const N: usize> AddAssign for Vector<K, N> {
    fn add_assign(&mut self, other: Self) {
        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<K: Sub<Output = K> + Default + Copy, const N: usize> Sub for Vector<K, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut data = self.data;
        data.iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a = *a - *b);
        Vector { data }
    }
}

impl<K: SubAssign + Clone + Copy, const N: usize> SubAssign for Vector<K, N> {
    fn sub_assign(&mut self, other: Self) {
        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(a, b)| *a -= *b);
    }
}

impl<K: Mul<Output = K> + Default + Copy, const N: usize> Mul<K> for Vector<K, N> {
    type Output = Self;

    fn mul(self, scalar: K) -> Self::Output {
        let mut data = self.data;
        data.iter_mut().for_each(|d| *d = *d * scalar);
        Vector { data }
    }
}

impl<K, const N: usize> MulAssign<K> for Vector<K, N>
where
    K: MulAssign + Clone + Copy,
{
    fn mul_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|d| *d *= scalar);
    }
}

impl<K: Clone + Copy, const N: usize> Index<usize> for Vector<K, N> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K: Clone + Copy, const N: usize> IndexMut<usize> for Vector<K, N> {
    fn index_mut(&mut self, index: usize) -> &mut K {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_assign_test() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        v1 += v2;
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }
    #[test]
    fn add_test() {
        let v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        let result = v1 + v2;
        assert_eq!(result, res, "add {:?} and {:?}", v1, res);
    }

    #[test]
    fn sub_assign_test() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        v1 -= v2;
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }
    #[test]
    fn sub_test() {
        let v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        let result = v1 - v2;
        assert_eq!(result, res, "add {:?} and {:?}", v1, res);
    }

    #[test]
    fn mul_assign_test() {
        let mut v1 = Vector::from([2., 3.]);
        let scalar = 2.;
        let res = Vector::from([4., 6.]);
        v1 *= scalar;
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }
    #[test]
    fn mul_test() {
        let v1: Vector<f32, 2> = Vector::from([2., 3.]);
        let scalar: f32 = 2.;
        let res = Vector::from([4., 6.]);
        let result = v1 * scalar;
        assert_eq!(result, res, "add {:?} and {:?}", v1, res);
    }
}
