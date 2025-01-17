use std::{array::IntoIter, iter::Flatten, slice::Iter};

pub struct AtMost<T, const N: usize> {
    pub inner: [Option<T>; N],
}

impl<T, const N: usize> AtMost<T, N> {
    pub fn new(xs: [T; N]) -> Self {
        let mut inner = [const { None }; N];
        for (x, i) in xs.into_iter().zip(inner.iter_mut()) {
            *i = Some(x);
        }
        Self { inner }
    }

    pub fn one(x: T) -> Self {
        let mut inner = [const { None }; N];
        inner[0] = Some(x);
        Self { inner }
    }

    pub fn some<I>(xs: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut inner = [const { None }; N];
        for (x, i) in xs.into_iter().zip(inner.iter_mut()) {
            *i = Some(x);
        }
        Self { inner }
    }
}

impl<T, const N: usize> IntoIterator for AtMost<T, N> {
    type Item = T;
    type IntoIter = Flatten<IntoIter<Option<T>, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().flatten()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a AtMost<T, N> {
    type Item = &'a T;
    type IntoIter = Flatten<Iter<'a, Option<T>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter().flatten()
    }
}
