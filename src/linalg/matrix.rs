pub mod operations;
pub mod iterators;
pub mod indexing;

use std::mem::{self, MaybeUninit};
use iterators::{MatrixIter, MatrixIterMut};

#[derive(Debug)]
pub struct Matrix<T: Default, const R: usize, const C: usize>(pub [[T; C]; R]);
impl<T: Default, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new() -> Self {
        let cells = {
            let mut data: [[mem::MaybeUninit<T>; C]; R] = unsafe {
                mem::MaybeUninit::uninit().assume_init()
            };

            for row in data.iter_mut() {
                for val in row.iter_mut() {
                    *val = MaybeUninit::new( T::default() );
                }
            }

            unsafe { mem::transmute_copy::<_, [[T; C]; R] >(&data) }
        }; 
        Matrix(cells) 
    }
    pub fn from( data: [[T; C]; R] ) -> Self {
        Matrix(data)
    }
    pub fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
    pub fn get(&self, (row, col): (usize, usize)) -> &T {
        &self.0[row][col]
    }
    pub fn get_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.0[row][col]
    }
    pub fn set(&mut self, (row, col): (usize, usize), value: T) {
        *self.get_mut((row, col)) = value;
    }
    pub fn replace(&mut self, (row, col): (usize, usize), value: T) -> T {
        mem::replace(self.get_mut( (row, col) ), value)
    }
    pub fn transpose(self) -> Matrix<T, C, R> {
        let mut m = Matrix::new();
        for (row_num, row) in self.0.into_iter().enumerate() {
            for (col_num, val) in row.into_iter().enumerate() {
                m[(col_num, row_num)] = val;
            }
        }
        m
    }
    pub fn iter(&self) -> MatrixIter<T, R, C> {
        MatrixIter::new(self)
    }
    pub fn iter_mut(&mut self) -> MatrixIterMut<T, R, C> {
        MatrixIterMut::new( self)
    }
    pub fn rows(self) -> std::array::IntoIter<[T; C], R> {
        self.0.into_iter()
    }
    pub fn cols(self) -> std::array::IntoIter<[T; R], C> {
        let mut_self = self;
        mut_self.transpose().0.into_iter()
    }
}

impl<T: Copy + Default, const R: usize, const C: usize> Copy for Matrix<T, R, C> {}
impl<T: Copy + Default, const R: usize, const C: usize> Clone for Matrix<T, R, C> {
    fn clone(&self) -> Self {
        let mut m: Matrix<T, R, C> = Matrix::new();
        for ((i, j), &v) in self.iter().enumerate() {
            m[(i, j)] = v;
        }
        m
    }
}

// unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let m = Matrix::from( [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(1, *m.get((0, 0)));
    }
    #[test]
    fn get_mut() {
        let mut m = Matrix::from( [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(&mut 1, m.get_mut((0, 0)));
    }
    #[test]
    fn set() {
        let mut m = Matrix::from( [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        m.set((0, 0), 2);
        assert_eq!(&2, m.get((0, 0)));
    }
    #[test]
    fn replace() {
        let mut m = Matrix::from([[String::from("One")]]);
        let old_val = m.replace((0, 0), String::from("Two"));
        assert_eq!(String::from("One"), old_val);
        assert_eq!(String::from("Two"), m[(0,0)]);
    }
}