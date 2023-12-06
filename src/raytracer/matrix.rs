use std::ops::{Index, IndexMut, Mul};

use float_cmp::approx_eq;

use super::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Matrix {
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

    pub fn identity(height: usize, width: usize) -> Self {
        let mut m = Self::new(height, width);
        for i in 0..height {
            m[(i, i)] = 1.0;
        }
        m
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn transpose(&self) -> Self {
        let mut out = Matrix::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                out[(x, y)] = self[(y, x)]
            }
        }
        out
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Self {
        let values = self.values.iter()
            .enumerate()
            .filter(|(i, _)| i / self.width != row && i % self.width != column)
            .map(|(_, v)| *v)
            .collect();
        let out = Matrix::from_values(self.height - 1, self.width - 1, values);
        out
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let mut out = self.submatrix(row, column).determinant();
        if (row + column) % 2 == 1 {
            out *= -1.0;
        }
        out
    }

    pub fn determinant(&self) -> f64 {
        if self.height != self.width {
            panic!("Can't calculate the determinant of a non-square matrix");
        }

        if self.height == 2 {
            return (self.values[0] * self.values[3]) - (self.values[1] * self.values[2]);
        }

        let mut out = 0.0;
        for x in 0..self.width {
            out += self.values[x] * self.cofactor(0, x)
        }
        out
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        let mut cofactor_matrix = Matrix::new(self.height, self.width);
        for y in 0..self.height {
            for x in 0..self.width {
                cofactor_matrix[(y, x)] = self.cofactor(y, x)
            }
        }
        let cofactor_matrix_t = cofactor_matrix.transpose();
        &cofactor_matrix_t * (1.0 / self.determinant())
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
        let mut out = Matrix::new(rhs.height(), self.width);
        for y in 0..self.height {
            for x in 0..rhs.width() {
                let mut sum = 0.0;
                for i in 0..self.width {
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

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let new_values = self.values.iter()
            .map(|v| v * rhs)
            .collect();
        Matrix::from_values(self.height, self.width, new_values)
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

    #[test]
    fn identity_creates_identity_matrix() {
        // When
        let m = Matrix::identity(4, 4);

        // Then
        assert_eq!(m, Matrix::from_values(4, 4, vec![
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0
        ]));
    }

    #[test]
    fn mul_matrix_by_identity_matrix_returns_matrix() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            0.0, 1.0, 2.0, 4.0, 
            1.0, 2.0, 4.0, 8.0, 
            2.0, 4.0, 8.0, 16.0, 
            4.0, 8.0, 16.0, 32.0
        ]);

        let b = Matrix::identity(4, 4);

        // When
        let c = &a * &b;

        // Then
        assert_eq!(c, Matrix::from_values(4, 4, vec![
            0.0, 1.0, 2.0, 4.0, 
            1.0, 2.0, 4.0, 8.0, 
            2.0, 4.0, 8.0, 16.0, 
            4.0, 8.0, 16.0, 32.0
        ]));
    }

    #[test]
    fn transpose_transposes_matrix() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            0.0, 9.0, 3.0, 0.0, 
            9.0, 8.0, 0.0, 8.0, 
            1.0, 8.0, 5.0, 3.0, 
            0.0, 0.0, 5.0, 8.0
        ]);

        // When
        let b = a.transpose();

        // Then
        assert_eq!(b, Matrix::from_values(4, 4, vec![
            0.0, 9.0, 1.0, 0.0, 
            9.0, 8.0, 8.0, 0.0, 
            3.0, 0.0, 5.0, 5.0, 
            0.0, 8.0, 3.0, 8.0
        ]));
    }

    #[test]
    fn transpose_identity_matrix_is_identity_matrix() {
        // Given
        let a = Matrix::identity(4, 4);

        // When
        let b = a.transpose();

        // Then
        assert_eq!(b, Matrix::identity(4, 4));
    }

    #[test]
    fn determinant_2x2_matrix_calculates_determinant() {
        // Given
        let a = Matrix::from_values(2, 2, vec![
            1.0, 5.0, 
            -3.0, 2.0
        ]);

        // When
        let d = a.determinant();

        // Then
        assert_eq!(d, 17.0);
    }

    #[test]
    fn submatrix_3x3_matrix_extracts_2x2_matrix() {
        // Given
        let a = Matrix::from_values(3, 3, vec![
            1.0, 5.0, 0.0, 
            -3.0, 2.0, 7.0, 
            0.0, 6.0, -3.0
        ]);

        // When
        let b = a.submatrix(0, 2);

        // Then
        assert_eq!(b, Matrix::from_values(2, 2, vec![
            -3.0, 2.0, 
            0.0, 6.0
        ]));
    }

    #[test]
    fn submatrix_4x4_matrix_extracts_3x3_matrix() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            -6.0, 1.0, 1.0, 6.0, 
            -8.0, 5.0, 8.0, 6.0, 
            -1.0, 0.0, 8.0, 2.0, 
            -7.0, 1.0, -1.0, 1.0
        ]);

        // When
        let b = a.submatrix(2, 1);

        // Then
        assert_eq!(b, Matrix::from_values(3, 3, vec![
            -6.0, 1.0, 6.0, 
            -8.0, 8.0, 6.0, 
            -7.0, -1.0, 1.0
        ]));
    }

    #[test]
    fn minor_3x3_matrix_calculates_minor() {
        // Given
        let a = Matrix::from_values(3, 3, vec![
            3.0, 5.0, 0.0, 
            2.0, -1.0, -7.0, 
            6.0, -1.0, 5.0
        ]);
        let b = a.submatrix(1, 0);

        // Then
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_3x3_matrix_calculates_cofactor() {
        // Given
        let a = Matrix::from_values(3, 3, vec![
            3.0, 5.0, 0.0, 
            2.0, -1.0, -7.0, 
            6.0, -1.0, 5.0
        ]);

        // Then
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_3x3_matrix_calculates_determinant() {
        // Given
        let a = Matrix::from_values(3, 3, vec![
            1.0, 2.0, 6.0, 
            -5.0, 8.0, -4.0, 
            2.0, 6.0, 4.0
        ]);

        // Then
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn determinant_4x4_matrix_calculates_determinant() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            -2.0, -8.0, 3.0, 5.0, 
            -3.0, 1.0, 7.0, 3.0, 
            1.0, 2.0, -9.0, 6.0, 
            -6.0, 7.0, 7.0, -9.0
        ]);

        // Then
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn is_invertible_returns_true_for_invertible_matrix() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            6.0, 4.0, 4.0, 4.0, 
            5.0, 5.0, 7.0, 6.0, 
            4.0, -9.0, 3.0, -7.0, 
            9.0, 1.0, 7.0, -6.0
        ]);

        // Then
        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn is_invertible_returns_false_for_non_invertible_matrix() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            -4.0, 2.0, -2.0, -3.0, 
            9.0, 6.0, 2.0, 6.0, 
            0.0, -5.0, 1.0, -5.0, 
            0.0, 0.0, 0.0, 0.0
        ]);

        // Then
        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn inverse_calculates_inverse() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            -5.0, 2.0, 6.0, -8.0, 
            1.0, -5.0, 1.0, 8.0, 
            7.0, 7.0, -6.0, -7.0, 
            1.0, -3.0, 7.0, 4.0
        ]);

        // When
        let b = a.inverse();

        // Then
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[(2, 3)], 105.0 / 532.0);
        assert_eq!(b, Matrix::from_values(4, 4, vec![
            0.21805, 0.45113, 0.24060, -0.04511, 
            -0.80827, -1.45677, -0.44361, 0.52068, 
            -0.07895, -0.22368, -0.05263, 0.19737, 
            -0.52256, -0.81391, -0.30075, 0.30639
        ]));
    }

    #[test]
    fn inverse_calculates_inverse_2() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            8.0, -5.0, 9.0, 2.0, 
            7.0, 5.0, 6.0, 1.0, 
            -6.0, 0.0, 9.0, 6.0, 
            -3.0, 0.0, -9.0, -4.0
        ]);

        // When
        let b = a.inverse();

        // Then
        assert_eq!(b, Matrix::from_values(4, 4, vec![
            -0.15385, -0.15385, -0.28205, -0.53846, 
            -0.07692, 0.12308, 0.02564, 0.03077, 
            0.35897, 0.35897, 0.43590, 0.92308, 
            -0.69231, -0.69231, -0.76923, -1.92308
        ]));
    }

    #[test]
    fn inverse_calculates_inverse_3() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            9.0, 3.0, 0.0, 9.0, 
            -5.0, -2.0, -6.0, -3.0, 
            -4.0, 9.0, 6.0, 4.0, 
            -7.0, 6.0, 6.0, 2.0
        ]);

        // When
        let b = a.inverse();

        // Then
        assert_eq!(b, Matrix::from_values(4, 4, vec![
            -0.04074, -0.07778, 0.14444, -0.22222, 
            -0.07778, 0.03333, 0.36667, -0.33333, 
            -0.02901, -0.14630, -0.10926, 0.12963, 
            0.17778, 0.06667, -0.26667, 0.33333
        ]));
    }

    #[test]
    fn mul_matrix_by_inverse_returns_identity_matrix() {
        // Given
        let a = Matrix::from_values(4, 4, vec![
            3.0, -9.0, 7.0, 3.0, 
            3.0, -8.0, 2.0, -9.0, 
            -4.0, 4.0, 4.0, 1.0, 
            -6.0, 5.0, -1.0, 1.0
        ]);

        let b = Matrix::from_values(4, 4, vec![
            8.0, 2.0, 2.0, 2.0, 
            3.0, -1.0, 7.0, 0.0, 
            7.0, 0.0, 5.0, 4.0, 
            6.0, -2.0, 0.0, 5.0
        ]);

        // When
        let c = &a * &b;

        // Then
        assert_eq!(&c * &b.inverse(), a);
    }
}
