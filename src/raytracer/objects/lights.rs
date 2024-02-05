use crate::raytracer::{color::Color, tuple::Tuple};

pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        Self { position, intensity }
    }

    pub fn position(&self) -> &Tuple {
        &self.position
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_members() {
        // Given
        let position = Tuple::point(0.0, 0.0, 0.0);
        let intensity = Color::new(1.0, 1.0, 1.0);

        // When
        let light = PointLight::new(position.clone(), intensity.clone());

        // Then
        assert_eq!(light.position(), &position);
        assert_eq!(light.intensity(), &intensity);
    }
}