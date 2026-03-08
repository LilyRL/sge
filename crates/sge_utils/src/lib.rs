use std::ops::{Index, IndexMut};

pub struct RotatingArray<T, const N: usize> {
    n: usize,
    arr: [T; N],
}

impl<T, const N: usize> RotatingArray<T, N> {
    pub fn new(arr: [T; N]) -> Self {
        Self { n: 0, arr }
    }

    pub fn push(&mut self, item: T) {
        self.arr[self.n] = item;
        self.n = (self.n + 1) % N;
    }

    pub fn get(&self, i: usize) -> &T {
        &self.arr[(self.n + i) % N]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.arr[(self.n + i) % N]
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..N).map(move |i| self.get(i))
    }

    pub fn current_value(&self) -> &T {
        self.get(N - 1)
    }

    pub fn previous_value(&self) -> &T {
        self.get(N - 2)
    }

    pub fn past_value(&self, i: usize) -> &T {
        self.get(N - 1 - i)
    }

    pub fn len(&self) -> usize {
        N
    }
}

impl<T, const N: usize> IntoIterator for RotatingArray<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

impl<T, const N: usize> Index<usize> for RotatingArray<T, N> {
    type Output = T;

    fn index(&self, n: usize) -> &Self::Output {
        self.get(n)
    }
}

impl<T, const N: usize> IndexMut<usize> for RotatingArray<T, N> {
    fn index_mut(&mut self, n: usize) -> &mut Self::Output {
        self.get_mut(n)
    }
}
