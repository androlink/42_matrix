mod operator;

mod add;
mod scale;
mod sub;

pub type Vec2 = Vector<f32, 2>;
pub type Vec3 = Vector<f32, 3>;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector<K, const N: usize> {
    data: [K; N],
}

impl<K: std::cmp::PartialEq, const N: usize> Vector<K, N> {
    pub fn size(&self) -> usize {
        N
    }
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(value: [K; N]) -> Self {
        Self { data: value }
    }
}

impl<K: Default + Copy, const N: usize> Default for Vector<K, N> {
    fn default() -> Self {
        Self {
            data: [K::default(); N],
        }
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
    #[test]
    fn test_sub() {
        let mut v1 = Vector::from([2., 3.]);
        let v2 = Vector::from([5., 7.]);
        let res = Vector::from([-3., -4.]);
        v1.sub(&v2);
        assert_eq!(v1, res, "sub {:?} and {:?}", v1, res);
    }

    #[test]
    fn test_scale() {
        let mut v1 = Vector::from([2., 3.]);
        let res = Vector::from([4., 6.]);
        v1.scl(2.);
        assert_eq!(v1, res, "scale {:?} and {:?}", v1, res);
    }
}
