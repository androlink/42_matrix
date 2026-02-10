use std::ops::MulAssign;

use super::Vector;

impl<K: Copy + MulAssign + Default, const N: usize> Vector<K, N> {
    pub fn scl(&mut self, a: K) {
        *self *= a;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scale() {
        let mut v1 = Vector::from([2., 3.]);
        let res = Vector::from([4., 6.]);
        v1.scl(2.);
        assert_eq!(v1, res, "scale {:?} and {:?}", v1, res);
    }
}
