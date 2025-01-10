use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone)]
pub struct MinHeap<T> {
    inner: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.inner.push(Reverse(t));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop().map(|x| x.0)
    }
}

impl<T: Ord> FromIterator<T> for MinHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().map(|t| Reverse(t)).collect(),
        }
    }
}
