use std::{array::IntoIter, iter::Flatten, ops::Range, slice::Iter};

// Visit all natural number points in a hyper rectangular prism.
#[derive(Debug)]
pub struct DynCartesianProduct {
    maxes: Vec<u32>,
    iters: Vec<Range<u32>>,
    state: Vec<u32>,
    len: u32,
    count: u32,
}

impl DynCartesianProduct {
    // Returns None if any of the maxes are 0.
    pub fn new(maxes: Vec<u32>) -> Option<Self> {
        let len: u32 = maxes.iter().copied().product();
        let mut iters: Vec<Range<u32>> = maxes.iter().map(|x| 0..*x).collect();
        let state: Option<Vec<u32>> = iters.iter_mut().map(|xs| xs.next()).collect();
        Some(Self {
            maxes,
            iters,
            state: state?,
            len,
            count: 0,
        })
    }
}

impl Iterator for DynCartesianProduct {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.len {
            return None;
        }

        let result = self.state.clone();
        let update_iter = self
            .state
            .iter_mut()
            .rev()
            .zip(self.iters.iter_mut().enumerate().rev());

        for (x, (i, range)) in update_iter {
            match range.next() {
                Some(new_value) => {
                    *x = new_value;
                    break;
                }
                None => {
                    *range = 0..(self.maxes[i]);
                    *x = range.next().expect("All ranges are non-empty");
                }
            }
        }

        self.count += 1;
        Some(result)
    }
}

// Like `zip`, but iterates though the second iterable multiple
// times to get all possible pairs from the two iterables.
pub struct CartesianProduct<CT, CU> {
    x: CT,
    y: CU,
}

impl<CT, CU> CartesianProduct<CT, CU> {
    pub fn new(x: CT, y: CU) -> Self {
        Self { x, y }
    }
}

pub struct CartesianProductIterator<IT, T, CU, IU> {
    outer: IT,
    current_outer: Option<T>,
    inner_iterable: CU,
    inner: IU,
}

impl<CT, IT, T, CU, IU, U> IntoIterator for CartesianProduct<CT, CU>
where
    T: Copy,
    IT: Iterator<Item = T>,
    IU: Iterator<Item = U>,
    CT: IntoIterator<IntoIter = IT, Item = T>,
    CU: IntoIterator<IntoIter = IU, Item = U> + Copy,
{
    type IntoIter = CartesianProductIterator<IT, T, CU, IU>;
    type Item = (T, U);

    fn into_iter(self) -> Self::IntoIter {
        let mut outer = self.x.into_iter();
        let current_outer = outer.next();
        CartesianProductIterator {
            outer,
            current_outer,
            inner_iterable: self.y,
            inner: self.y.into_iter(),
        }
    }
}

impl<IT, T, CU, IU, U> Iterator for CartesianProductIterator<IT, T, CU, IU>
where
    T: Copy,
    IT: Iterator<Item = T>,
    CU: IntoIterator<IntoIter = IU, Item = U> + Copy,
    IU: Iterator<Item = U>,
{
    type Item = (T, U);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(current_inner) => {
                let current_outer = self.current_outer?;
                Some((current_outer, current_inner))
            }
            None => {
                self.inner = self.inner_iterable.into_iter();
                self.current_outer = self.outer.next();

                let current_inner = self.inner.next()?;
                let current_outer = self.current_outer?;
                Some((current_outer, current_inner))
            }
        }
    }
}

pub struct TakeN<'a, I> {
    n: usize,
    taken: usize,
    inner: &'a mut I,
}

impl<'a, I> TakeN<'a, I>
where
    I: Iterator,
{
    pub fn new(iter: &'a mut I, n: usize) -> Self {
        Self {
            n,
            taken: 0,
            inner: iter,
        }
    }
}

impl<'a, I, T> Iterator for TakeN<'a, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.taken == self.n {
            return None;
        }

        let item = self.inner.next()?;
        self.taken += 1;
        Some(item)
    }
}

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

    pub fn push(&mut self, x: T) {
        for q in self.inner.iter_mut() {
            if q.is_none() {
                *q = Some(x);
                break;
            }
        }
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

#[test]
fn test_cartesian_product() {
    let xs = vec![0u8, 1, 2, 3];
    let ys = vec![7u8, 8];

    let product: Vec<(u8, u8)> = CartesianProduct::new(xs, &ys)
        .into_iter()
        .map(|(x, y)| (x, *y))
        .collect();

    assert_eq!(
        product,
        vec![
            (0, 7),
            (0, 8),
            (1, 7),
            (1, 8),
            (2, 7),
            (2, 8),
            (3, 7),
            (3, 8),
        ]
    );

    let xs = vec![0u8, 1];
    let ys = vec![2u8];
    let zs = vec![3u8, 4, 5];

    let product: Vec<(u8, u8, u8)> = CartesianProduct::new(CartesianProduct::new(xs, &ys), &zs)
        .into_iter()
        .map(|((x, y), z)| (x, *y, *z))
        .collect();

    assert_eq!(
        product,
        vec![
            (0, 2, 3),
            (0, 2, 4),
            (0, 2, 5),
            (1, 2, 3),
            (1, 2, 4),
            (1, 2, 5),
        ],
    );
}

#[test]
fn test_dyn_cartesian_product() {
    let product: Vec<Vec<u32>> = DynCartesianProduct::new(vec![3, 1, 2]).unwrap().collect();
    assert_eq!(
        product,
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![1, 0, 0],
            vec![1, 0, 1],
            vec![2, 0, 0],
            vec![2, 0, 1],
        ],
    );

    let product: Vec<Vec<u32>> = DynCartesianProduct::new(vec![4]).unwrap().collect();
    assert_eq!(product, vec![vec![0], vec![1], vec![2], vec![3],]);

    let product: Vec<Vec<u32>> = DynCartesianProduct::new(Vec::new()).unwrap().collect();
    assert_eq!(product, vec![vec![]],);
}
