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
