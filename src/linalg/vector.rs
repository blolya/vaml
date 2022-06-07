pub mod indexing;
pub mod operations;
pub mod iterators;

#[derive(Debug)]
pub struct Vector<T, const L: usize>([T; L]);
impl<T, const L: usize> Vector<T, L> {
    pub fn from(data: [T; L]) -> Self {
        Vector( data )
    }
    pub fn get(&self, index: usize) -> &T {
        &self.0[index]
    }
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
    pub fn set(&mut self, index: usize, value: T) {
        self.0[index] = value;
    }
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.0.iter_mut()
    }
}

impl<T: Copy, const L: usize> Copy for Vector<T, L> {}
impl<T: Copy, const L: usize> Clone for Vector<T, L> {
    fn clone(&self) -> Self {
       Vector::from(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let m = Vector::from( [1, 2, 3]);
        assert_eq!(1, *m.get(0));
    }
    #[test]
    fn get_mut() {
        let mut m = Vector::from( [1, 2, 3]);
        assert_eq!(&mut 1, m.get_mut(0));
    }
    #[test]
    fn set() {
        let mut m = Vector::from( [1, 2, 3]);
        m.set( 0, 2);
        assert_eq!(&2, m.get(0));
    }
}
