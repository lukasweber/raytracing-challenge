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

pub fn rotation_x(radians: f64) -> Matrix {
    let mut out = Matrix::new(4,4);
    out[(0, 0)] = 1.0;
    out[(1, 1)] = radians.cos();
    out[(1, 2)] = -radians.sin();
    out[(2, 1)] = radians.sin();
    out[(2, 2)] = radians.cos();
    out[(3, 3)] = 1.0;
    out
}

pub fn rotation_y(radians: f64) -> Matrix {
    let mut out = Matrix::new(4,4);
    out[(0, 0)] = radians.cos();
    out[(0, 2)] = radians.sin();
    out[(2, 0)] = -radians.sin();
    out[(1, 1)] = 1.0;
    out[(2, 2)] = radians.cos();
    out[(3, 3)] = 1.0;
    out
}

pub fn rotation_z(radians: f64) -> Matrix {
    let mut out = Matrix::new(4,4);
    out[(0, 0)] = radians.cos();
    out[(0, 1)] = -radians.sin();
    out[(1, 0)] = radians.sin();
    out[(1, 1)] = radians.cos();
    out[(2, 2)] = 1.0;
    out[(3, 3)] = 1.0;
    out
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut out = Matrix::identity(4, 4);
    out[(0, 1)] = xy;
    out[(0, 2)] = xz;
    out[(1, 0)] = yx;
    out[(1, 2)] = yz;
    out[(2, 0)] = zx;
    out[(2, 1)] = zy;
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

    #[test]
    fn rotation_x_rotates_point_around_x_axis() {
        // Given
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_x(std::f64::consts::PI / 2.0);

        // When
        let p2 = &half_quarter * &p;
        let p3 = &full_quarter * &p;

        // Then
        assert_eq!(p2, Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0));
        assert_eq!(p3, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        // Given
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let inv = half_quarter.inverse();

        // When
        let p2 = &inv * &p;

        // Then
        assert_eq!(p2, Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
    }

    #[test]
    fn rotation_y_rotates_point_around_y_axis() {
        // Given
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_y(std::f64::consts::PI / 2.0);

        // When
        let p2 = &half_quarter * &p;
        let p3 = &full_quarter * &p;

        // Then
        assert_eq!(p2, Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0));
        assert_eq!(p3, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    pub fn rotation_z_rotates_point_around_z_axis() {
        // Given
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(std::f64::consts::PI / 4.0);
        let full_quarter = rotation_z(std::f64::consts::PI / 2.0);

        // When
        let p2 = &half_quarter * &p;
        let p3 = &full_quarter * &p;

        // Then
        assert_eq!(p2, Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0));
        assert_eq!(p3, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        // Given
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        // Given
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        // Given
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        // Given
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(2.0, 7.0, 4.0));
    }
    
    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        // Given
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        // Given
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        // When
        let p2 = &transform * &p;

        // Then
        assert_eq!(p2, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_applied_in_sequence() {
        // Given
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        // When
        let p2 = &a * &p;
        let p3 = &b * &p2;
        let p4 = &c * &p3;

        // Then
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_applied_in_reverse_order() {
        // Given
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = &c * &b * &a;

        // When
        let p2 = &t * &p;

        // Then
        assert_eq!(p2, Tuple::point(15.0, 0.0, 7.0));
    }
}
