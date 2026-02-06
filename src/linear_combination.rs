use crate::vector::Vector;

// V1 * C1 + V2 * C2 + ... + Vs * Cs;

pub fn linear_combination<K, const N: usize, const S: usize>(
    u: &[Vector<K, N>; S],
    coefs: &[K; S],
) -> Vector<K, N>
where
    K: Clone + Copy + Default + std::ops::Mul<Output = K>,
    Vector<K, N>: std::ops::Mul<K, Output = Vector<K, N>> + std::ops::AddAssign,
{
    let mut v: Vector<K, N> = Vector::default();
    for i in 0..S {
        v += u[i] * coefs[i];
    }
    v
}

// pub fn linear_combination<const N: usize, const S: usize>(u: &[Vector<f32, N>; S], coefs: &[f32; S]) -> Vector<f32, N>
// {
//   let mut v : Vector<f32, N> = Vector::from([0.; N]);

//   for index in 0..N
//   {
//     let mut result:f32 = 0.;
//     for vec_id in 0..S
//     {
//       result = u[vec_id].data[index].mul_add(coefs[vec_id], result);
//     }
//     v.data[index] = result
//   }
//   v
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_combination() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        let assert_res1 = Vector::from([10., -2., 0.5]);
        let assert_res2 = Vector::from([10., 0., 230.]);

        let res = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);
        assert_eq!(res, assert_res1, "{:?} not {:?}", res, assert_res1);
        let res = linear_combination(&[v1, v2], &[10., -2.]);
        assert_eq!(res, assert_res2, "{:?} not {:?}", res, assert_res2);
    }
}
