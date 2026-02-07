use std::ops::MulAssign;

use super::Vector;

impl<K: Copy + MulAssign + Default, const N: usize> Vector<K, N> {
    pub fn scl(&mut self, a: K) {
        *self *= a;
    }
}
