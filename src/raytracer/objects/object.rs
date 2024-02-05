use crate::raytracer::{matrix::Matrix, tuple::Tuple};

use super::{intersection::Intersection, ray::Ray};

pub trait Object { 
    fn intersects(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_at(&self, point: &Tuple) -> Tuple;
    fn transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);
}
