
use std::ops::AddAssign;

use super::Vector;

impl<K: AddAssign + Copy, const N: usize> Vector<K, N> {
    pub fn add(&mut self, v: &Vector<K, N>) {
      *self += *v;
    }
}
