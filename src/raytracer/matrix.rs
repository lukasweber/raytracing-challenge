use std::ops::{Index, IndexMut, Mul};

use float_cmp::approx_eq;

use super::tuple::Tuple;

#[derive(Debug, Clone)]
struct Matrix {
    height: usize,
    width: usize,
    values: Vec<f64>
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width, values: vec![0.0; height * width] }
    }

    pub fn from_values(height: usize, width: usize, values: Vec<f64>) -> Self {
        Self { height, width, values }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
       &self.values[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.values[y * self.width + x]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && 
        self.width == other.width && 
        self.values.iter()
            .zip(other.values.iter())
            .all(|(a, b)| approx_eq!(f64, *a, *b, epsilon=0.00001))
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut out = Matrix::new(rhs.height(), self.width());
        for y in 0..self.height() {
            for x in 0..rhs.width() {
                let mut sum = 0.0;
                for i in 0..self.width() {
                    sum += self[(y, i)] * rhs[(i, x)];
                }
                out[(y, x)] = sum;
            }
        }
        out
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        let out = self * &Matrix::from_values(4, 1, vec![rhs.x(), rhs.y(), rhs.z(), rhs.w()]);
        Tuple::new(out[(0, 0)], out[(1, 0)], out[(2, 0)], out[(3, 0)])
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn new_initializes_members() {
        // When
        let m = Matrix::new(4, 4);

        // Then
        assert_eq!(m.height(), 4);
        assert_eq!(m.width(), 4);
        assert_eq!(m.values.len(), 16);
        assert!(m.values.iter().all(|v| *v == 0.0));
    }

    #[test]
    fn index_4x4_matrix_reads_values() {
        // Given & When
        let m = Matrix::from_values(4, 4, vec![
            1.0, 2.0, 3.0, 4.0, 
            5.5, 6.5, 7.5, 8.5, 
            9.0, 10.0, 11.0, 12.0, 
            13.5, 14.5, 15.5, 16.5
        ]);

        // Then
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn index_mut_2x2_matrix_assigns_values() {
        // Given
        let mut m = Matrix::new(2, 2);

        // When
        m[(0, 0)] = -3.0;
        m[(0, 1)] = 5.0;
        m[(1, 0)] = 1.0;
        m[(1, 1)] = -2.0;

        // Then
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }

    #[test]
    fn index_3x3_matrix_reads_values() {
        // Given
        let m = Matrix::from_values(3, 3, vec![
            -3.0, 5.0, 0.0, 
            1.0, -2.0, -7.0, 
            0.0, 1.0, 1.0
        ]);

        // When & Then
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn eq_compares_members() {
        // Given
        let create_matrix = |height, width| {
            let mut m = Matrix::new(height, width);
            for y in 0..height {
                for x in 0..width {
                    m[(y, x)] = (y * width + x) as f64;
                }
            }
            m
        };
        let a = create_matrix(4, 4);
        let b = create_matrix(4, 4);
        
        // When & Then
        assert_eq!(a, b);
    }

    #[test]
    fn ne_compares_members() {
        // Given
        let a = Matrix::new(4, 4);
        let mut b = Matrix::new(4, 4);
        b[(0, 0)] = 100.0;
        
        // When & Then
        assert_ne!(a, b);
    }

    #[test]
    fn mul_multiplies_matrices() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            1.0, 2.0, 3.0, 4.0, 
            5.0, 6.0, 7.0, 8.0, 
            9.0, 8.0, 7.0, 6.0, 
            5.0, 4.0, 3.0, 2.0
        ]);

        let b = Matrix::from_values(4, 4, vec![
            -2.0, 1.0, 2.0, 3.0, 
            3.0, 2.0, 1.0, -1.0, 
            4.0, 3.0, 6.0, 5.0, 
            1.0, 2.0, 7.0, 8.0
        ]);

        // When
        let c = &a * &b;

        // Then
        assert_eq!(c, Matrix::from_values(4, 4, vec![
            20.0, 22.0, 50.0, 48.0, 
            44.0, 54.0, 114.0, 108.0, 
            40.0, 58.0, 110.0, 102.0, 
            16.0, 26.0, 46.0, 42.0
        ]));
    }

    #[test]
    fn mul_multiplies_matrix_by_tuple() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            1.0, 2.0, 3.0, 4.0, 
            2.0, 4.0, 4.0, 2.0, 
            8.0, 6.0, 4.0, 1.0, 
            0.0, 0.0, 0.0, 1.0
        ]);

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        // When
        let c = &a * &b;

        // Then
        assert_eq!(c, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }    
}
