use super::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut out = Matrix::identity(4, 4);
    out[(0, 3)] = x;
    out[(1, 3)] = y;
    out[(2, 3)] = z;
    out
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut out = Matrix::new(4,4);
    out[(0, 0)] = x;
    out[(1, 1)] = y;
    out[(2, 2)] = z;
    out[(3, 3)] = 1.0;
    out
}

#[cfg(test)]
mod tests {
    use crate::raytracer::tuple::Tuple;

    use super::*;

    #[test]
    fn translation_matrix_translates_point() {
        // Given
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn mul_by_inverse_of_translation_matrix_translates_point_back() {
        // Given
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        // When
        let p2 = &inv * &p;

        // Then
        assert_eq!(p2, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_matrix_doesnt_affect_vector() {
        // Given
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        // When
        let v2 = &transform * &v;

        // Then
        assert_eq!(v2, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        // Given
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        // Given
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        // When
        let v2 = &transform * &v;

        // Then
        assert_eq!(v2, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn mul_by_inverse_of_scaling_matrix() {
        // Given
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        // When
        let v2 = &inv * &v;

        // Then
        assert_eq!(v2, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        // Given
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(-2.0, 3.0, 4.0));
    }
}
