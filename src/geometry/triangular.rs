use std::cmp::Ordering;

use crate::{
    Aabb, geometry::intersect::Intersect, geometry::intersection::Intersection, geometry::ray::Ray,
    geometry::triangle::Triangle,
};

pub(crate) struct Triangular {
    triangles: Vec<Triangle>,
    aabb: Aabb,
}

impl Triangular {
    pub(crate) fn builder() -> TriangularBuilder {
        TriangularBuilder::default()
    }
}

impl Intersect for Triangular {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.aabb.intersect(ray)?;
        self.triangles
            .iter()
            .flat_map(|triangle| triangle.intersect(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }
}

#[derive(Default)]
pub(crate) struct TriangularBuilder {
    triangles: Vec<Triangle>,
}

impl TriangularBuilder {
    pub(crate) fn add_triangle(mut self, triangle: Triangle) -> Self {
        self.triangles.push(triangle);
        self
    }

    pub(crate) fn build(self) -> Triangular {
        let triangles = self.triangles;
        let triangle_count = triangles.len();
        let aabb = match triangle_count {
            0 => Aabb::default(),
            1 => triangles[0].aabb(),
            _ => triangles
                .iter()
                .map(|triangle| triangle.aabb())
                .reduce(|acc, e| acc.extended(e))
                .unwrap_or_default(),
        };
        Triangular { triangles, aabb }
    }
}
