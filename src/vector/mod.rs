use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

mod operator;

mod add;
mod scale;
mod sub;

mod dot_product;
mod norm;

pub type Vec2 = Vector<f32, 2>;
pub type Vec3 = Vector<f32, 3>;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector<K, const N: usize> {
    data: [K; N],
}

impl<K, const N: usize> Display for Vector<K, N>
where
    K: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl<K: std::cmp::PartialEq, const N: usize> Vector<K, N> {
    pub fn size(&self) -> usize {
        N
    }
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(value: [K; N]) -> Self {
        Self { data: value }
    }
}

impl<K: Default + Copy, const N: usize> Default for Vector<K, N> {
    fn default() -> Self {
        Self {
            data: [K::default(); N],
        }
    }
}

impl<K, const N: usize> Deref for Vector<K, N> {
    type Target = [K; N];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<K, const N: usize> DerefMut for Vector<K, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
