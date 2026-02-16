use std::ops::{Mul, Sub};

use crate::vector::Vector;

pub fn cross_product<K>(u: Vector<K, 3>, v: Vector<K, 3>) -> Vector<K, 3>
where
    K: Copy,
    K: Mul<Output = K>,
    K: Sub<Output = K>,
{
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
    .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cross_product() {
        let u = Vector::from([0., 0., 1.]);
        let v = Vector::from([1., 0., 0.]);
        assert_eq!(cross_product(u, v), [0., 1., 0.].into());
        // [0.]
        // [1.]
        // [0.]
        let u = Vector::from([1., 2., 3.]);
        let v = Vector::from([4., 5., 6.]);
        assert_eq!(cross_product(u, v), [-3., 6., -3.].into());
        // [-3.]
        // [6.]
        // [-3.]
        let u = Vector::from([4., 2., -3.]);
        let v = Vector::from([-2., -5., 16.]);
        assert_eq!(cross_product(u, v), [17., -58., -16.].into());
        // [17.]
        // [-58.]
        // [-16.]
    }
}
