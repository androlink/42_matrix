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
    use crate::{matrix::Matrix, vector::Vector};

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

    #[test]
    fn test_linear_interpolation_vector() {
        let v1 = Vector::from([2., 1.]);
        let v2 = Vector::from([4., 2.]);
        let scalar = 0.3;
        let res = lerp(v1, v2, scalar);
        let wanted = [2.6, 1.3].into();
        assert_eq!(res, wanted, "testing {:?}, {:?}, {:?} and {:?}", v1, v2, scalar, wanted);
    }

    #[test]
    fn test_linear_interpolation_matrix() {
        let v1 = Matrix::from([[2., 1.], [3., 4.]]);
        let v2 = Matrix::from([[20.,10.], [30., 40.]]);
        let scalar = 0.5;
        let res = lerp(v1, v2, scalar);
        let wanted = [[11., 5.5],[16.5, 22.]].into();
        assert_eq!(res, wanted, "testing {:?}, {:?}, {:?} and {:?}", v1, v2, scalar, wanted);
    }
}