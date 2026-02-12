use std::{fmt::Debug, ops::{Add, Mul}};

use super::Vector;

impl<K, const N: usize> Vector<K, N>
where
    K: Default + Debug + Mul<Output = K> + Add<Output = K> + Clone + Copy,
{
    pub fn dot(&self, v: Self) -> K {
        self.data
            .iter()
            .zip(v.data.iter())
            .fold(K::default(), |acc, (a, b)| acc + *a * *b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dot() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        let res = u.dot(v);
        assert_eq!(res, 0.0, "assert {:?}, receive {:?}", 0.0, res);
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        let res = u.dot(v);
        assert_eq!(res, 2.0, "assert {:?}, receive {:?}", 2.0, res);
        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        let res = u.dot(v);
        assert_eq!(res, 9.0, "assert {:?}, receive {:?}", 9.0, res);
    }
}
