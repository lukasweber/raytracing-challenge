use crate::raytracer::{matrix::Matrix, tuple::Tuple};

use super::{ray::Ray, object::Object, intersection::Intersection};



pub struct Sphere { 
    transform: Matrix
}

impl Sphere {
    fn intersects(&self, ray: &Ray) -> Vec<Intersection> {
        let ray = ray.transform(&self.transform.inverse());

        let sphere_to_ray = ray.origin() - &Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

        let t1_intersection = Intersection::new(t1, self);
        let t2_intersection = Intersection::new(t2, self);
        
        vec![t1_intersection, t2_intersection]
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Object for Sphere { }

impl Default for Sphere {
    fn default() -> Sphere {
        Self { transform: Matrix::identity(4, 4) }
    }
}

#[cfg(test)]
mod tests {
    use crate::raytracer::transformation;

    use super::*;

    #[test]
    fn intersects_ray_at_two_points() {
        // Given
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        // When
        let xs = s.intersects(&r);

        // Then
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 6.0);
    }

    #[test]
    fn intersects_ray_at_tangent() {
        // Given
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();
        
        // When
        let xs = s.intersects(&r);

        // Then
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 5.0);
        assert_eq!(xs[1].t(), 5.0);

    }

    #[test]
    fn intersects_ray_misses() {
        // Given
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        // When
        let xs = s.intersects(&r);

        // Then
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersects_ray_inside_sphere() {
        // Given
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        // When
        let xs = s.intersects(&r);

        // Then
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -1.0);
        assert_eq!(xs[1].t(), 1.0);
    }

    #[test]
    fn intersects_ray_in_front_of_sphere() {
        // Given
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        // When
        let xs = s.intersects(&r);

        // Then
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -6.0);
        assert_eq!(xs[1].t(), -4.0);
    }

    #[test]
    fn transform_returns_default() {
        // Given
        let s = Sphere::default();

        // When
        let t = s.transform();

        // Then
        assert_eq!(t, &Matrix::identity(4, 4));
    }

    #[test]
    fn set_transform_sets_member() {
        // Given
        let mut s = Sphere::default();
        let t = transformation::translation(2.0, 3.0, 4.0);

        // When
        s.set_transform(t.clone());

        // Then
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn intersects_applies_scale_transform() {
        // Given
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        // When
        s.set_transform(transformation::scaling(2.0, 2.0, 2.0));
        let xs = s.intersects(&r);

        // Then
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 3.0);
        assert_eq!(xs[1].t(), 7.0);
    }

    #[test]
    fn intersects_applies_translate_transform() {
        // Given
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        // When
        s.set_transform(transformation::translation(5.0, 0.0, 0.0));
        let xs = s.intersects(&r);

        // Then
        assert_eq!(xs.len(), 0);
    }
}
