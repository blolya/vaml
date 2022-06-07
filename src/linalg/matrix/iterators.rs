
use super::Matrix;
use std::iter::Iterator;
pub struct MatrixIterator<T: Default, const R: usize, const C: usize> {
    matrix: Matrix<T, R, C>,
    iter_index: (usize, usize),
    size: (usize, usize)
}
impl<T: Default, const R: usize, const C: usize> MatrixIterator<T, R, C> {
    pub fn new(matrix: Matrix<T, R, C>) -> Self {
        MatrixIterator {
            size: matrix.size(),
            matrix,
            iter_index: (0, 0),
        }
    }
    pub fn enumerate(self) -> MatrixEnumerator<T, R, C> {
        MatrixEnumerator::new(self)
    }
}
impl<T: Default, const R: usize, const C: usize> Iterator for MatrixIterator<T, R, C> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (row_total, col_total) = self.size;
        let (cur_row, cur_col) = self.iter_index;

        if cur_row == row_total {
            self.iter_index = (0, 0);
            return None;
        }

        let elem = std::mem::replace(self.matrix.get_mut((cur_row, cur_col)), T::default()); 
        let (cur_row, cur_col) = if cur_col + 1 == col_total {
            (cur_row + 1, 0 as usize)
        } else {
            (cur_row, cur_col + 1)
        };

        self.iter_index = (cur_row, cur_col);
        Some(elem)       
    }
}
pub struct MatrixEnumerator<T: Default, const R: usize, const C: usize>(MatrixIterator<T, R, C>);
impl<T: Default, const R: usize, const C: usize> MatrixEnumerator<T, R, C> {
    fn new(matrix: MatrixIterator<T, R, C>) -> Self {
        MatrixEnumerator(matrix)
    }
}
impl<T: Default, const R: usize, const C: usize> Iterator for MatrixEnumerator<T, R, C> {
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0.iter_index;
        if let Some(v) = self.0.next() {
            Some((index, v))
        } else {
            None
        }
    }
}

use std::iter::IntoIterator;
impl<T: Default, const R: usize, const C: usize> IntoIterator for Matrix<T, R, C> {
    type Item = T;
    type IntoIter = MatrixIterator<T, R, C>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIterator::new(self)
    }
}
use std::iter::FromIterator;
impl<T: Default, const R: usize, const C: usize> FromIterator<T> for Matrix<T, R, C> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut m: Matrix<T, R, C> = Matrix::new();
        let mut m_iter = m.iter_mut();

        for v in iter {
            if let Some(mut val) = m_iter.next() {
                *val = v;
            }
        }
        m
    }
}
pub struct MatrixIter<'a, T: Default, const R: usize, const C: usize> {
    matrix: &'a Matrix<T, R, C>,
    iter_index: (usize, usize),
    size: (usize, usize)
}
impl<'a, T: Default, const R: usize, const C: usize> MatrixIter<'a, T, R, C> {
    pub fn new(matrix: &'a Matrix<T, R, C>) -> Self {
        MatrixIter {
            size: matrix.size(),
            matrix,
            iter_index: (0, 0),
        }
    }
    pub fn enumerate(self) -> MatrixEnum<'a, T, R, C> {
        MatrixEnum::new(self)
    }
}
impl<'a, T: Default, const R: usize, const C: usize> Iterator for MatrixIter<'a, T, R, C> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (row_total, col_total) = self.size;
        let (cur_row, cur_col) = self.iter_index;

        if cur_row == row_total {
            self.iter_index = (0, 0);
            return None;
        }

        let elem = &self.matrix[(cur_row, cur_col)];
        let (cur_row, cur_col) = if cur_col + 1 == col_total {
            (cur_row + 1, 0 as usize)
        } else {
            (cur_row, cur_col + 1)
        };

        self.iter_index = (cur_row, cur_col);
        Some(elem)       
    }
}

pub struct MatrixEnum<'a, T: Default, const R: usize, const C: usize>(MatrixIter<'a, T, R, C>);
impl<'a, T: Default, const R: usize, const C: usize> MatrixEnum<'a, T, R, C> {
    pub fn new(matrix: MatrixIter<'a, T, R, C>) -> Self {
        MatrixEnum(matrix)
    }
}
impl<'a, T: Default, const R: usize, const C: usize> Iterator for MatrixEnum<'a, T, R, C> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0.iter_index;
        if let Some(v) = self.0.next() {
            Some((index, v))
        } else {
            None
        }
    }
}

use std::ops::{Deref, DerefMut};
pub struct MutWrapper<T> {
    pointer: *mut T,
}
impl<T> MutWrapper<T> {
    fn new(pointer: *mut T) -> Self {
        MutWrapper { pointer }
    }
}
impl<T> Deref for MutWrapper<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
       unsafe { self.pointer.as_ref().unwrap() }
    }
}

impl<T> DerefMut for MutWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
       unsafe { self.pointer.as_mut().unwrap() }
    }
}
pub struct MatrixIterMut<'a, T: Default, const R: usize, const C: usize> {
    matrix: &'a mut Matrix<T, R, C>,
    iter_index: (usize, usize),
    size: (usize, usize)
}
impl<'a, T: Default, const R: usize, const C: usize> MatrixIterMut<'a, T, R, C> {
    pub fn new(matrix: &'a mut Matrix<T, R, C>) -> Self {
        MatrixIterMut {
            size: matrix.size(),
            matrix,
            iter_index: (0, 0),
        }
    }
    pub fn enumerate(self) -> MatrixEnumMut<'a, T, R, C> {
        MatrixEnumMut::new(self)
    }
}
impl<'a, T: Default, const R: usize, const C: usize> Iterator for MatrixIterMut<'a, T, R, C> {
    type Item = MutWrapper<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (row_total, col_total) = self.size;
        let (cur_row, cur_col) = self.iter_index;

        if cur_row == row_total {
            self.iter_index = (0, 0);
            return None;
        }

        let elem = MutWrapper::new( &mut self.matrix[(cur_row, cur_col)] as *mut T );
        let (cur_row, cur_col) = if cur_col + 1 == col_total {
            (cur_row + 1, 0 as usize)
        } else {
            (cur_row, cur_col + 1)
        };

        self.iter_index = (cur_row, cur_col);
        Some(elem)       
    }
}

pub struct MatrixEnumMut<'a, T: Default, const R: usize, const C: usize>(MatrixIterMut<'a, T, R, C>);
impl<'a, T: Default, const R: usize, const C: usize> MatrixEnumMut<'a, T, R, C> {
    pub fn new(matrix: MatrixIterMut<'a, T, R, C>) -> Self {
        MatrixEnumMut(matrix)
    }
}
impl<'a, T: Default, const R: usize, const C: usize> Iterator for MatrixEnumMut<'a, T, R, C> {
    type Item = ((usize, usize), MutWrapper<T>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0.iter_index;
        if let Some(v) = self.0.next() {
            Some((index, v))
        } else {
            None
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let m = Matrix::from([[1, 2, 3], [4, 5, 6]]);
        let mut sum = 0;
        for v in m.iter() {
            sum += *v;
        }
        assert_eq!(sum, 21);
    }
    #[test]
    fn iter_mut() {
        let mut m = Matrix::from([[1]]);
        for mut v in m.iter_mut() {
           *v = 2;
        }
        assert_eq!(m.get((0,0)), &2);
    }
    #[test]
    fn enumer() {
        let m = Matrix::from([[1, 2], [4, 5]]);
        let mut indexes = vec![];
        for ((row, col), _) in m.iter().enumerate() {
            indexes.push( (row, col) );
        }
        assert_eq!(indexes, [(0, 0), (0, 1), (1, 0), (1, 1)]);
    }
    #[test]
    fn enumer_mut() {
        let mut m = Matrix::from([[1]]);
        for ((_, _), mut v) in m.iter_mut().enumerate() {
            *v = 2;
        }
        assert_eq!(m.get((0,0)), &2);
    }
    #[test]
    fn from_iter() {
        let iter = vec![1, 2, 3, 4, 5];
        let test = Matrix::from([[1, 2], [3, 4]]);
        let m: Matrix<_, 2, 2> = Matrix::from_iter(iter.into_iter());
        assert_eq!(m, test);
    }
}