use crate::vector::Vector;

impl<K: Copy + std::ops::SubAssign<K>, const N: usize> Vector<K, N> {
    pub fn sub(&mut self, v: &Vector<K, N>) {
        let mut i: usize = 0;
        while i < self.data.len() {
            self.data[i] -= v.data[i];
            i += 1;
        }
    }
}
