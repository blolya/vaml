use super::Matrix;
use std::ops::{Index, IndexMut};

impl<T: Default, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index)
    }
}
impl<T: Default, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index)
    }
}

// tests

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn index() {
        let m = Matrix::from([[String::from("One")]]);
        assert_eq!(String::from("One"), m[(0,0)]);
    }

    #[test]
    fn index_mut() {
        let mut m = Matrix::from([[String::from("One")]]);
        m[(0, 0)] = String::from("Two");
        assert_eq!(String::from("Two"), m[(0,0)]);
    }

}