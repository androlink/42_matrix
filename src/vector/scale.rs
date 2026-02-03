use crate::vector::Vector;

impl<K: Copy + std::ops::MulAssign<K>, const N: usize> Vector<K, N> {
    pub fn scl(&mut self, a: K) {
        let mut i = 0;
        while i < self.data.len() {
            self.data[i] *= a;
            i += 1;
        }
    }
}
