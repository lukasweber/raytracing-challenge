use crate::raytracer::{matrix::Matrix, ray::Ray, tuple::Tuple};

use super::intersection::Intersection;
use super::materials::Material;

pub trait Object { 
    fn intersects(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, point: &Tuple) -> Tuple;
    fn transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);
    fn material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
}
