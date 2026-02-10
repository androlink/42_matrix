use super::Vector;

impl<K: Copy + std::ops::SubAssign<K>, const N: usize> Vector<K, N> {
    pub fn sub(&mut self, v: &Vector<K, N>) {
        let mut i: usize = 0;
        while i < self.data.len() {
            self.data[i] -= v.data[i];
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sub() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        v1.sub(&v2);
        assert_eq!(v1, res, "sub {:?} and {:?}", v1, res);
    }
}
