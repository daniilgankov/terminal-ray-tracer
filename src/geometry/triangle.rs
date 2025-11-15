use crate::{
    Aabb, Vec3f,
    consts::EPSILON,
    geometry::{intersect::Intersect, intersection::Intersection, ray::Ray},
};

pub(crate) struct Triangle {
    a: Vec3f,
    b: Vec3f,
    c: Vec3f,
    // Store normal to improve performance
    normal: Vec3f,
}

impl Triangle {
    pub(crate) fn new(a: Vec3f, b: Vec3f, c: Vec3f) -> Self {
        let normal = compute_normal(a, b, c);
        Self { a, b, c, normal }
    }

    pub(crate) fn aabb(&self) -> Aabb {
        let min = self.a.min(self.b).min(self.c);
        let max = self.a.max(self.b).max(self.c);
        Aabb { min, max }
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let d_cross_ac = ray.direction.cross(ac);
        let determinant = ab.dot(d_cross_ac);
        if determinant.abs() < EPSILON {
            return None;
        }
        let inverse_determinant = 1.0 / determinant;
        let ao = ray.origin - self.a;
        let u = inverse_determinant * ao.dot(d_cross_ac);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let ao_cross_ab = ao.cross(ab);
        let v = inverse_determinant * ray.direction.dot(ao_cross_ab);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = inverse_determinant * ac.dot(ao_cross_ab);
        (t > EPSILON).then_some(Intersection {
            distance: t,
            normal: self.normal,
        })
    }
}

fn compute_normal(a: Vec3f, b: Vec3f, c: Vec3f) -> Vec3f {
    let ab = b - a;
    let ac = c - a;
    ab.cross(ac).normalize()
}
