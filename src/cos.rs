use crate::vector::Vector;

pub fn angle_cos<K, const N: usize>(u: &Vector<K, N>, v: &Vector<K, N>) -> f32
where
    K: Clone,
    K: Copy,
    K: Default,
    K: std::ops::Add<Output = K>,
    K: std::ops::Mul<Output = K>,
    K: std::ops::Div<f32, Output = f32>,
    f32: From<K>,
{
    u.dot(*v) / (u.norm() * v.norm())
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_float_eq::assert_float_relative_eq;

    #[test]
    fn test_cos() {
        let u = Vector::from([0., 1.]);
        let v = Vector::from([0., 1.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 1.0);

        let u = Vector::from([1., 0.]);
        let v = Vector::from([0., 1.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 0.);

        let u = Vector::from([-1., 1.]);
        let v = Vector::from([1., -1.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, -1.);

        let u = Vector::from([2., 1.]);
        let v = Vector::from([4., 2.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 1.);

        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        let res = angle_cos(&u, &v);
        assert_float_relative_eq!(res, 0.974631846);
    }
}
