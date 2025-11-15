use crate::{
    consts::EPSILON,
    geometry::{intersect::Intersect, intersection::Intersection, ray::Ray},
    math::vec3::Vec3f,
};

#[derive(Default)]
pub(crate) struct Aabb {
    pub(crate) min: Vec3f,
    pub(crate) max: Vec3f,
}

impl Aabb {
    pub(crate) fn centered(center: Vec3f, face_distance: f32) -> Self {
        let min = center - face_distance;
        let max = center + face_distance;
        Self { min, max }
    }

    pub(crate) fn extended(self, other: Self) -> Self {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Self { min, max }
    }
}

impl Intersect for Aabb {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        // The time each component needs to walk a unit of distance
        let ray_delta = 1.0 / ray.direction;
        let time_to_min = ray_delta * (self.min - ray.origin);
        let time_to_max = ray_delta * (self.max - ray.origin);
        let fastest = time_to_min.min(time_to_max);
        let slowest = time_to_min.max(time_to_max);
        // Near is the max fastest time because its point must be on the last intersected axis
        // plane. It is expected to be reached from outside of the box (from the inside, the value
        // is just negative). The opposite applies to the far which is on the first intersected
        // axis plane.
        let near = fastest.max_component();
        let far = slowest.min_component();
        (far > 0.0 && near < far).then_some(Intersection {
            distance: near - EPSILON,
            normal: -ray.direction.signum() * fastest.step(near),
        })
    }
}
