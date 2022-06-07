use super::Vector;

pub trait Sqrt {
    type Output;

    fn sqrt(val: Self::Output) -> Self::Output; 
}

impl Sqrt for f64 {
    type Output = f64;

    fn sqrt(val: f64) -> Self::Output {
        Self::sqrt(val)
    }
}
impl Sqrt for f32 {
    type Output = f32;

    fn sqrt(val: f32) -> Self::Output {
        Self::sqrt(val)
    }
}

impl<T, const L: usize> Vector<T, L> 
where
    T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Default + Copy + Sqrt<Output = T>
{
    pub fn norm(&self) -> T {
        T::sqrt(*self * *self)
    }
    pub fn dist(&self, rhs: &Self) -> T {
        T::sqrt(
            self.into_iter()
                .zip( rhs.into_iter() )
                .fold( T::default(), |mut acc, (v1, v2)| { 
                    let diff = v1 - v2;
                    acc = acc + diff * diff;
                    acc
                })
        )
    }
}

use std::ops::{Mul, Add, Sub, Div};

impl<T, const L: usize> Vector<T, L>
where
    T: Add<Output = T>
{
    pub fn add(self, rhs: Self) -> Self {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(v1, v2)| v1 + v2)
            .collect()
    }
}
impl<T, const L: usize> Add for Vector<T, L>
where
    T: Add<Output = T>
{
    type Output = Vector<T, L>;
    fn add(self, rhs: Self) -> Self::Output {
       self.add(rhs) 
    }
}

impl<T, const L: usize> Vector<T, L>
where
    T: Sub<Output = T>
{
    pub fn sub(self, rhs: Self) -> Self {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(v1, v2)| v1 - v2)
            .collect()
    }
}
impl<T, const L: usize> Sub for Vector<T, L>
where
    T: Sub<Output = T>
{
    type Output = Vector<T, L>;
    fn sub(self, rhs: Self) -> Self::Output {
       self.sub(rhs) 
    }
}

impl<T, const L: usize> Vector<T, L>
where
    T: Mul<Output = T> + Copy
{
    pub fn mul_scal(self, rhs: T) -> Self {
        self.into_iter()
            .map( |v| v * rhs )
            .collect()
    }
}
impl<T, const L: usize> Vector<T, L>
where
    T: Div<Output = T> + Copy
{
    pub fn div_scal(self, rhs: T) -> Self {
        self.into_iter()
            .map( |v| v / rhs )
            .collect()
    }
}

impl<T, const L: usize> Vector<T, L>
where
    T: Add<Output = T> + Mul<Output = T> + Default
{
    pub fn dot(self, rhs: Self) -> T {
        self.into_iter()
            .zip(rhs.into_iter())
            .fold(T::default(), |mut acc, (v1, v2)| { acc = acc + v1 * v2; acc } )
    }
}
impl<T, const L: usize> Mul for Vector<T, L>
where
    T: Mul<Output = T> + Add<Output = T> + Default
{
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
       self.dot(rhs) 
    }
}

impl<T> Vector<T, 3>
where
    T: Sub<Output = T> + Mul<Output = T> + Default + Copy
{
    pub fn cross(self, rhs: Self) -> Self {
        let x = self[1] * rhs[2] - self[2] * rhs[1];
        let y = self[2] * rhs[0] - self[0] * rhs[2];
        let z = self[0] * rhs[1] - self[1] * rhs[0];
        Vector::from([x, y, z])
    }
}
