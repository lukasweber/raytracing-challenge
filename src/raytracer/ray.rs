use super::tuple::Tuple;

pub struct Ray {
    origin: Tuple,
    direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Tuple {
        &self.origin
    }

    pub fn direction(&self) -> &Tuple {
        &self.direction
    }

    pub fn position(&self, t: f64) -> Tuple {
        &self.origin + &self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_members() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin.clone(), direction.clone());

        assert_eq!(r.origin(), &origin);
        assert_eq!(r.direction(), &direction);
    }

    #[test]
    fn position_returns_point_at_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }
}
