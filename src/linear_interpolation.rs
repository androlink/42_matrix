use std::ops::{AddAssign, MulAssign, SubAssign};

/**
 *
 * time complexity: O(N)
 */
pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: SubAssign + Copy + MulAssign<f32> + AddAssign,
{
    let mut ret_value: V = v;
    ret_value -= u;
    ret_value *= t;
    ret_value += u;
    ret_value
}
