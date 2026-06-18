use crate::vector::Vector;

/**
 *
 * complexity: O(N)
 */
pub fn linear_combination<K, const N: usize, const S: usize>(
    u: &[Vector<K, N>; S],
    coefs: &[K; S],
) -> Vector<K, N>
where
    K: Clone + Copy + Default + std::ops::Mul<Output = K> + std::ops::Add,
    Vector<K, N>: std::ops::Mul<K, Output = Vector<K, N>> + std::ops::Add<Output = Vector<K, N>>,
{
    u.iter()
        .zip(coefs.iter())
        .fold(Vector::default(), |acc, (v, c)| acc + *v * *c)
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
