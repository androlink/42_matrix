use super::Vector;

impl<K: std::ops::AddAssign + Copy, const N: usize> Vector<K, N> {
    pub fn add(&mut self, v: &Vector<K, N>) {
        *self += *v;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([7., 10.]);
        v1.add(&v2);
        assert_eq!(v1, res, "add {:?} and {:?}", v1, res);
    }
}
