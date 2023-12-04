use std::ops;

use super::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Color {
    tuple: Tuple
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Self { tuple: Tuple::new(red, green, blue, 0.0) }
    }

    pub fn red(&self) -> f64 {
        self.tuple.x()
    }

    pub fn green(&self) -> f64 {
        self.tuple.y()
    }

    pub fn blue(&self) -> f64 {
        self.tuple.z()
    }

    pub fn hadamard_product(self, other: &Self) -> Color {
        self * other
    }

}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.tuple == other.tuple
    }
}

impl ops::Add<&Color> for Color {
    type Output = Color;

    fn add(self, other: &Color) -> Color {
        Color { tuple: self.tuple + &other.tuple }
    }
}

impl ops::Sub<&Color> for Color {
    type Output = Color;

    fn sub(self, other: &Color) -> Color {
        Color { tuple: self.tuple - &other.tuple }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color { tuple: self.tuple * rhs }
    }
}

impl ops::Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Color {
        Color::new(self.red() * rhs.red(), self.green() * rhs.green(), self.blue() * rhs.blue())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_members() {
        // Given
        let c = Color::new(-0.5, 0.4, 1.7);

        // When & Then
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn add_adds_members() {
        // Given
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        // When + Then
        assert_eq!(c1 + &c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn sub_subtracts_members() {
        // Given
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        // When & Then
        assert_eq!(c1 - &c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul_multiplies_members() {
        // Given
        let c = Color::new(0.2, 0.3, 0.4);

        // When & Then
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn mul_multiplies_colors() {
        // Given
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        // When & Then
        assert_eq!(c1 * &c2, Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn hadamard_product_multiplies_members() {
        // Given
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        // When & Then
        assert_eq!(c1.hadamard_product(&c2), Color::new(0.9, 0.2, 0.04));
    }
}
