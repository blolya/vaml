use super::Vector;
use std::ops::{Index, IndexMut};

impl<T, const L: usize> Index<usize> for Vector<T, L> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
       self.get(index) 
    }
}
impl<T, const L: usize> IndexMut<usize> for Vector<T, L> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

use std::ops::Range;
impl<T, const L: usize> Index<Range<usize>> for Vector<T, L> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}
impl<T, const L: usize> IndexMut<Range<usize>> for Vector<T, L> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}