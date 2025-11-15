use crate::geometry::{intersection::Intersection, ray::Ray};

pub(crate) trait Intersect {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}
