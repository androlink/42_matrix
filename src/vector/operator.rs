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

impl<K: Div<Output = K> + Default + Copy, const N: usize> Div<K> for Vector<K, N> {
    type Output = Self;

    fn div(self, scalar: K) -> Self::Output {
        self.data.map(|d| d / scalar).into()
    }
}

impl<K, const N: usize> DivAssign<K> for Vector<K, N>
where
    K: DivAssign + Clone + Copy,
{
    fn div_assign(&mut self, scalar: K) {
        self.data.iter_mut().for_each(|d| *d /= scalar);
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
