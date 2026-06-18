use crate::vector::Vector;

/**
 *
 * time complexity: O(N)
 */
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
