use super::Vector;

impl<K, const N: usize> Vector<K, N>
where
    K: Copy + std::ops::Mul<Output = K> + Into<f32>,
{
    pub fn norm_1(&self) -> f32 {
        self.data
            .iter()
            .fold(f32::default(), |acc, &a| a.into().abs() + acc)
    }
    pub fn norm(&self) -> f32 {
        self.data
            .iter()
            .fold(f32::default(), |acc, &a| (a * a).into() + acc)
            .sqrt()
    }
    pub fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .fold(f32::default(), |acc, &a| a.into().abs().max(acc))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_norm_1() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(
            u.norm_1(),
            0.0,
            "assert {:?}, receive {:?}",
            0.0,
            u.norm_1()
        );
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(
            u.norm_1(),
            6.0,
            "assert {:?}, receive {:?}",
            6.0,
            u.norm_1()
        );
        let u = Vector::from([-1., -2.]);
        assert_eq!(
            u.norm_1(),
            3.0,
            "assert {:?}, receive {:?}",
            3.0,
            u.norm_1()
        );
    }

    #[test]
    fn test_norm() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm(), 0.0, "assert {:?}, receive {:?}", 0.0, u.norm());
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(
            u.norm(),
            3.74165738,
            "assert {:?}, receive {:?}",
            3.74165738,
            u.norm()
        );
        let u = Vector::from([-1., -2.]);
        assert_eq!(
            u.norm(),
            2.236067977,
            "assert {:?}, receive {:?}",
            2.236067977,
            u.norm()
        );
    }

    #[test]
    fn test_norm_inf() {
        let u = Vector::from([0., 0., 0.]);
        assert_eq!(
            u.norm_inf(),
            0.0,
            "assert {:?}, receive {:?}",
            0.0,
            u.norm_inf()
        );
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(
            u.norm_inf(),
            3.0,
            "assert {:?}, receive {:?}",
            3.0,
            u.norm_inf()
        );
        let u = Vector::from([-1., -2.]);
        assert_eq!(
            u.norm_inf(),
            2.0,
            "assert {:?}, receive {:?}",
            2.0,
            u.norm_inf()
        );
    }
}
