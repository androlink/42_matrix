use std::ops::{AddAssign, MulAssign, SubAssign};

pub fn lerp<V>(u:V, v:V, t:f32) -> V
where V: SubAssign + Copy + MulAssign<f32> + AddAssign
{
    let mut ret_value: V = v;
    ret_value -= u;
    ret_value *= t;
    ret_value += u;
    ret_value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_interpolation() {
    }

    #[test]
    fn test_linear_interpolation_basic() {
        assert_eq!(lerp(0., 1., 0.), 0., "testing {:?} and {:?}", [0., 1., 0.], 0.);
        assert_eq!(lerp(0., 1., 1.), 1., "testing {:?} and {:?}", [0., 1., 1.], 1.);
        assert_eq!(lerp(0., 1., 0.5), 0.5, "testing {:?} and {:?}", [0., 1., 0.5], 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3, "testing {:?} and {:?}", [21., 42., 0.3], 27.3);
    }
}