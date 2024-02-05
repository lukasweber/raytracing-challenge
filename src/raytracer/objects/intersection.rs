use super::object::Object;

pub struct Intersection<'a> {
    t: f64,
    object: &'a dyn Object,
}

impl Intersection<'_> {

    pub fn new(t: f64, object: &dyn Object) -> Intersection {
        Intersection { t, object }
    }

    pub fn from_hit<'a>(intersections: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
        intersections.iter()
            .filter(|i| i.t() >= 0.0)
            .min_by(|a, b| a.t().partial_cmp(&b.t()).unwrap())
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &dyn Object {
        self.object
    }
}

#[cfg(test)]
mod tests {
    use crate::raytracer::objects::sphere::Sphere;

    use super::*;

    #[test]
    fn new_sets_members() {
        let t = 3.5;
        let s = Sphere::default();
        let i = Intersection::new(t, &s);

        assert_eq!(i.t(), t);
        assert!(std::ptr::eq(i.object(), &s));
    }

    #[test]
    fn from_hit_all_intersections_positive() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let intersections = vec![i1, i2];

        let i = Intersection::from_hit(&intersections);

        assert!(i.is_some());
        assert_eq!(i.unwrap().t(), 1.0);
    }

    #[test]
    fn from_hit_some_intersections_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let intersections = vec![i1, i2];

        let i = Intersection::from_hit(&intersections);

        assert!(i.is_some());
        assert_eq!(i.unwrap().t(), 1.0);
    }

    #[test]
    fn from_hit_all_intersections_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let intersections = vec![i1, i2];

        let i = Intersection::from_hit(&intersections);

        assert!(i.is_none());
    }

    #[test]
    fn from_hit_lowest_non_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let intersections = vec![i1, i2, i3, i4];

        let i = Intersection::from_hit(&intersections);

        assert!(i.is_some());
        assert_eq!(i.unwrap().t(), 2.0);
    }
}