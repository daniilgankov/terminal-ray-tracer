use std::cmp::Ordering;

use crate::{consts::EPSILON, geometry::ray::Ray, math::vec3::Vec3f};

#[derive(Clone, Copy)]
pub(crate) struct Intersection {
    pub(crate) distance: f32,
    pub(crate) normal: Vec3f,
}

impl Intersection {
    pub(crate) fn hit_position(&self, ray: Ray) -> Vec3f {
        let distance = self.distance - EPSILON;
        ray.origin + distance * ray.direction
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}
