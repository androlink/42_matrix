use super::Vector;

impl<K, const N: usize> Vector<K, N>
where
    K: Copy + std::ops::Mul<Output = K> + Into<f32>,
{
    /**
     *
     * time complexity: O(N)
     */
    pub fn norm_1(&self) -> f32 {
        self.data
            .iter()
            .fold(f32::default(), |acc, &a| a.into().abs() + acc)
    }

    /**
     *
     * time complexity: O(N)
     */
    pub fn norm(&self) -> f32 {
        self.data
            .iter()
            .fold(f32::default(), |acc, &a| (a * a).into() + acc)
            .sqrt()
    }

    /**
     *
     * time complexity: O(N)
     */
    pub fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .fold(f32::default(), |acc, &a| a.into().abs().max(acc))
    }
}
