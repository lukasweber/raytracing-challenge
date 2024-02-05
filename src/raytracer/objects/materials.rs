use float_cmp::approx_eq;

use crate::raytracer::{color::Color, tuple::Tuple};

use super::lights::PointLight;

#[derive(Debug, Clone)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Material {
        Self { color, ambient, diffuse, specular, shininess }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse;
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular;
    }

    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = shininess;
    }

    pub fn lighting(&self, light: &PointLight, position: &Tuple, eye_vec: &Tuple, normal_vec: &Tuple) -> Color {
        // combine the surface color with the light's color/intensity
        let effective_color = &self.color * light.intensity();

        // find the direction to the light source
        let lightv = (light.position() - position).normalize();

        // compute the ambient contribution
        let ambient = &effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between the light vector and the normal vector. 
        // A negative number means the light is on the other side of the surface.
        let light_dot_normal = lightv.dot(normal_vec);

        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);

        if light_dot_normal >= 0.0 {
            // compute the diffuse contribution
            diffuse = &effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the reflection vector and the eye vector. 
            // A negative number means the light reflects away from the eye.
            let reflectv = (-lightv).reflect(normal_vec);
            let reflect_dot_eye = reflectv.dot(eye_vec);

            if reflect_dot_eye > 0.0 {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity() * self.specular * factor;
            }
        }

        // add the three contributions together to get the final shading
        &ambient + &diffuse + &specular
    }


}

impl Default for Material {
    fn default() -> Material {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.ambient, other.ambient, epsilon = 0.00001) &&
        approx_eq!(f64, self.diffuse, other.diffuse, epsilon = 0.00001) &&
        approx_eq!(f64, self.specular, other.specular, epsilon = 0.00001) &&
        approx_eq!(f64, self.shininess, other.shininess, epsilon = 0.00001) &&
        self.color == other.color
    }
}

impl PartialEq<Material> for &Material {
    fn eq(&self, other: &Material) -> bool {
        *self == other
    }
}

#[cfg(test)]
mod tests {
    use crate::raytracer::{objects::lights::PointLight, tuple::Tuple};

    use super::*;

    #[test]
    fn new_sets_members() {
        // Given
        let color = Color::new(1.0, 1.0, 1.0);
        let ambient = 0.1;
        let diffuse = 0.9;
        let specular = 0.9;
        let shininess = 200.0;

        // When
        let m = Material::new(color.clone(), ambient, diffuse, specular, shininess);

        // Then
        approx_eq!(f64, m.ambient, ambient, epsilon = 0.00001);
        approx_eq!(f64, m.diffuse, diffuse, epsilon = 0.00001);
        approx_eq!(f64, m.specular, specular, epsilon = 0.00001);
        approx_eq!(f64, m.shininess, shininess, epsilon = 0.00001);
        assert_eq!(m.color(), &color);
    }

    #[test]
    fn lighnting_with_eye_between_light_and_surface() {
        // Given
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        // When
        let result = m.lighting(&light, &position, &eyev, &normalv);

        // Then
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighnting_with_eye_between_light_and_surface_eye_offset_45_degrees() {
        // Given
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 2_f64.sqrt()/2.0, -2_f64.sqrt()/2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        // When
        let result = m.lighting(&light, &position, &eyev, &normalv);

        // Then
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighnting_with_eye_opposite_surface_light_offset_45_degrees() {
        // Given
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        // When
        let result = m.lighting(&light, &position, &eyev, &normalv);

        // Then
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighnting_with_eye_in_path_of_reflection_vector() {
        // Given
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, -2_f64.sqrt()/2.0, -2_f64.sqrt()/2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        // When
        let result = m.lighting(&light, &position, &eyev, &normalv);

        // Then
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighnting_with_light_behind_surface() {
        // Given
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        // When
        let result = m.lighting(&light, &position, &eyev, &normalv);

        // Then
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

}