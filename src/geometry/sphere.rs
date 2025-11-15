use crate::{
    geometry::{intersect::Intersect, intersection::Intersection, ray::Ray},
    math::vec3::Vec3f,
};

pub(crate) struct Sphere {
    pub(crate) center: Vec3f,
    pub(crate) radius: f32,
}

impl Intersect for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let origin_to_center = self.center - ray.origin;
        // Let "mid" to be the middle between the intersection points
        let origin_to_mid_len = origin_to_center.dot(ray.direction);
        if origin_to_mid_len < 0.0 {
            return None;
        }
        let origin_to_center_len = origin_to_center.length();
        let center_to_mid_len_sqr =
            origin_to_center_len * origin_to_center_len - origin_to_mid_len * origin_to_mid_len;
        let radius_sqr = self.radius * self.radius;
        if center_to_mid_len_sqr < 0.0 || center_to_mid_len_sqr > radius_sqr {
            return None;
        }
        let intersection_to_mid_distance = (radius_sqr - center_to_mid_len_sqr).sqrt();
        let origin_to_intersection = origin_to_mid_len - intersection_to_mid_distance;
        let intersection = ray.origin + ray.direction * origin_to_intersection;
        let center_to_intersection = intersection - self.center;
        Some(Intersection {
            distance: origin_to_intersection,
            normal: center_to_intersection.normalize(),
        })
    }
}
