use super::Matrix;

use std::cmp::PartialEq;
impl<T: Default + PartialEq, const R: usize, const C: usize> PartialEq for Matrix<T, R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .enumerate()
            .all( |((row, col), v)| *v == other[(row, col)])
    }
}

use std::ops::Add;
impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Default + Add<Output = T>   
{
    pub fn add(self, rhs: Self) -> Self {
        let mut m = Matrix::new();
        let mut self_mut = self;
        let mut rhs_mut = rhs;
        for ((row, col), mut val) in m.iter_mut().enumerate() {
            *val = self_mut.replace((row, col), T::default()) + rhs_mut.replace((row, col), T::default());
        }
        m
    }
}
impl<T, const R: usize, const C: usize> Add<Matrix<T, R, C>> for Matrix<T, R, C>
where
    T: Default + Add<Output = T>
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

use std::ops::Mul;
impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Default + Mul<Output = T> + Copy + Add<Output = T>
{
    pub fn mul<const K: usize>(self, rhs: Matrix<T, C, K>) -> Matrix<T, R, K> {
        let mut m = Matrix::new();
        for (row_num, row) in self.rows().enumerate() {
            for (col_num, col) in rhs.cols().enumerate() {
                m[(row_num, col_num)] = row.iter()
                    .zip(col.iter())
                    .map(|(&v1, &v2)| {v1 * v2})
                    .fold(T::default(), |acc, v| acc + v);
            }
        }
        m
    }
}
impl<T, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>> for Matrix<T, R, C>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T>
{
    type Output = Matrix<T, R, K>;

    fn mul(self, rhs: Matrix<T, C, K>) -> Self::Output {
        self.mul(rhs)
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Default + Mul<Output = T> + Copy + Add<Output = T>
{
    pub fn mul_scal(&self, rhs: T) -> Matrix<T, R, C> {
        self.clone()
            .into_iter()
            .map(|v| v * rhs)
            .collect::<Matrix<T, R, C>>()
    }
}

impl<T, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T>
{
    type Output = Matrix<T, R, C>;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul_scal(rhs)
    }
}

// unit tests

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let m = Matrix::from([[1]]);
        let m2 = m;
        assert_eq!(m, m2);
    }
    #[test]
    fn add() {
        let m1 = Matrix::from([[1, 1], [1, 1]]);
        let m2 = m1;
        let m = Matrix::from([[2, 2], [2, 2]]);
        assert_eq!(m, m1 + m2);
    }

    #[test]
    fn mul() {
        let m1 = Matrix::from([[1, 1, 1], [1, 1, 1]]);
        let m2 = m1.clone().transpose();
        let m = Matrix::from([[3, 3], [3, 3]]);
        assert_eq!(m, m1 * m2);
    }
    #[test]
    fn mul_scal() {
        let m1 = Matrix::from([[1, 1], [1, 1]]);
        let m = Matrix::from([[2, 2], [2, 2]]);
        assert_eq!(m, m1 * 2);
    }
}