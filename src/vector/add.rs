
use super::Vector;

impl<K: std::ops::AddAssign + Copy, const N: usize> Vector<K, N> {
    pub fn add(&mut self, v: &Vector<K, N>) {
      *self += *v;
    }
}
