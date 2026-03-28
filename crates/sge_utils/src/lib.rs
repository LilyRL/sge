use std::ops::{Add, Index, IndexMut, Mul, Sub};

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

    pub fn is_empty(&self) -> bool {
        N == 0
    }
}

impl<T: ToF32 + FromF32 + Copy, const N: usize> RotatingArray<T, N> {
    pub fn avg(&self) -> T {
        let sum = self.iter().fold(0.0, |acc, &item| acc + item.to_f32());
        FromF32::from_f32(sum / N as f32)
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

pub trait Lerpable:
    Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self> + Sized + Copy
{
    fn lerp(self, other: Self, progress: f32) -> Self {
        self + (other - self) * progress
    }
}

impl<T> Lerpable for T where
    T: Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f32, Output = Self> + Sized + Copy
{
}

pub struct Lerped<T: Lerpable> {
    start: T,
    end: T,
}

impl<T: Lerpable> Lerped<T> {
    pub fn new(start: T, end: T) -> Lerped<T> {
        Lerped { start, end }
    }

    pub fn value(&self, progress: f32) -> T {
        self.start.lerp(self.end, progress)
    }

    pub fn now_offset_towards(&mut self, new_end: T) {
        self.start = self.end;
        self.end = new_end;
    }
}

pub trait ToF32 {
    fn to_f32(self) -> f32;
}

impl ToF32 for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl ToF32 for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for isize {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for usize {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

pub trait FromF32 {
    fn from_f32(value: f32) -> Self;
}

impl FromF32 for f32 {
    fn from_f32(value: f32) -> Self {
        value
    }
}

impl FromF32 for f64 {
    fn from_f32(value: f32) -> Self {
        value as f64
    }
}

impl FromF32 for i32 {
    fn from_f32(value: f32) -> Self {
        value as i32
    }
}

impl FromF32 for u32 {
    fn from_f32(value: f32) -> Self {
        value as u32
    }
}

impl FromF32 for i16 {
    fn from_f32(value: f32) -> Self {
        value as i16
    }
}

impl FromF32 for u16 {
    fn from_f32(value: f32) -> Self {
        value as u16
    }
}

impl FromF32 for i8 {
    fn from_f32(value: f32) -> Self {
        value as i8
    }
}

impl FromF32 for u8 {
    fn from_f32(value: f32) -> Self {
        value as u8
    }
}

impl FromF32 for isize {
    fn from_f32(value: f32) -> Self {
        value as isize
    }
}

impl FromF32 for usize {
    fn from_f32(value: f32) -> Self {
        value as usize
    }
}

pub trait PartialClamp: PartialOrd + Sized {
    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl<T: PartialOrd + Sized> PartialClamp for T {}
