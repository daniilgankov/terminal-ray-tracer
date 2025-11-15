use crate::{
    color::Color, geometry::intersect::Intersect, geometry::intersection::Intersection,
    geometry::ray::Ray,
};

pub(crate) struct Object {
    pub(crate) color: Color,
    pub(crate) intersect: Box<dyn Intersect + Send + Sync>,
}

impl Intersect for Object {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.intersect.intersect(ray)
    }
}
