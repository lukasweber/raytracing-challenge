use std::{ops::{self}, fmt::Display};

use float_cmp::approx_eq;

#[derive(Debug, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Self::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Self::new(x, y, z, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let magnitude = self.magnitude();
        Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude, self.w / magnitude)
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Self) -> Tuple {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, n: &Tuple) -> Tuple {
        self - &(&(n * 2.0) * self.dot(n))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x, epsilon=0.00001) &&
        approx_eq!(f64, self.y, other.y, epsilon=0.00001) &&
        approx_eq!(f64, self.z, other.z, epsilon=0.00001) &&
        approx_eq!(f64, self.w, other.w, epsilon=0.00001)
    }
}

// Add Operator

impl ops::Add<&Tuple> for &Tuple {
    type Output = Tuple;

    fn add(self, rhs: &Tuple) -> Self::Output {
        Tuple { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl ops::Add<&Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: &Tuple) -> Self::Output {
        &self + rhs
    }
}

impl ops::Add<Tuple> for &Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        self + &rhs
    }
}

// Sub Operator

impl ops::Sub<&Tuple> for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: &Tuple) -> Self::Output {
        Tuple { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl ops::Sub<&Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: &Tuple) -> Self::Output {
        &self - rhs
    }
}

// Neg Operator

impl ops::Neg for &Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        -&self
    }
}

// Mul Operator

impl ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

// Div Operator

impl ops::Div<f64> for &Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = if self.is_point() { "point" } else { "vector" };
        write!(f, "{} [{:.6}, {:.6}, {:.6}, {:.6}]", t, self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;

    #[test]
    fn new_point_sets_members() {
        // When
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        // Then
        assert!(approx_eq!(f64, a.x, 4.3, epsilon=0.00001));
        assert!(approx_eq!(f64, a.y, -4.2, epsilon=0.00001));
        assert!(approx_eq!(f64, a.z, 3.1, epsilon=0.00001));
        assert!(approx_eq!(f64, a.w, 1.0, epsilon=0.00001));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn new_vector_sets_members() {
        // When
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        // Then
        assert!(approx_eq!(f64, a.x, 4.3, epsilon=0.00001));
        assert!(approx_eq!(f64, a.y, -4.2, epsilon=0.00001));
        assert!(approx_eq!(f64, a.z, 3.1, epsilon=0.00001));
        assert!(approx_eq!(f64, a.w, 0.0, epsilon=0.00001));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuple() {
        // When
        let p = Tuple::point(4.0, -4.0, 3.0);

        // Then
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuple() {
        // When
        let v = Tuple::vector(4.0, -4.0, 3.0);

        // Then
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn add_adds_memebers() {
        // Given
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        // When
        let a3 = &a1 + &a2;

        // Then
        assert_eq!(a3, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtract_points_creates_vector() {
        // Given
        let a1 = Tuple::point(3.0, 2.0, 1.0);
        let a2 = Tuple::point(5.0, 6.0, 7.0);

        // When
        let a3 = &a1 - &a2;

        // Then
        assert_eq!(a3, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn substract_vector_from_point_creates_point() {
        // Given
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        // When
        let a3 = &p - &v;

        // Then
        assert_eq!(a3, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vectors_creates_vector() {
        // Given
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        // When
        let a3 = &v1 - &v2;

        // Then
        assert_eq!(a3, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_zero_vector_creates_vector() {
        // Given
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        // When
        let a3 = &zero - &v;

        // Then
        assert_eq!(a3, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_tuple() {
        // Given
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        // When
        let a2 = -&a;

        // Then
        assert_eq!(a2, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn mul_scalar_sets_members() {
        // Given
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        // When
        let a2 = &a * 3.5;

        // Then
        assert_eq!(a2, Tuple::new(3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    fn mul_fraction_sets_members() {
        // Given
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        // When
        let a2 = &a * 0.5;

        // Then
        assert_eq!(a2, Tuple::new(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn div_scalar_sets_members() {
        // Given
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        // When
        let a2 = a / 2.0;

        // Then
        assert_eq!(a2, Tuple::new(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn magnitude_calculates_correctly() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalize_calculates_correctly() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(approx_eq!(f64, v.normalize().magnitude(), 1.0, epsilon=0.00001));
    }

    #[test]
    fn dot_product_calculates_correctly() {
        // Given
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        // When & Then
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product_calculates_correctly() {
        // Given
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        // When & Then
        assert_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_vector_approaching_at_45_degrees() {
        // Given
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);

        // When
        let r = v.reflect(&n);

        // Then
        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        // Given
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);

        // When
        let r = v.reflect(&n);

        // Then
        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
