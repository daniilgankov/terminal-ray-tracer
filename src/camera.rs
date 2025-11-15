use crate::{
    geometry::ray::Ray,
    math::{
        mat4::Mat4f,
        vec2::Vec2f,
        vec3::{Vec3f, vec3},
        vec4::vec4,
    },
};

pub(crate) struct Camera {
    pub(crate) look_from: Vec3f,
    pub(crate) look_at: Vec3f,
}

pub(crate) const WORLD_UP: Vec3f = vec3!(0.0, 1.0, 0.0);

impl Camera {
    fn view_ray(&self) -> Ray {
        Ray {
            origin: self.look_from,
            direction: (self.look_at - self.look_from).normalize(),
        }
    }

    fn camera_to_world(&self) -> Mat4f {
        let view_ray = self.view_ray();
        let forward = view_ray.direction;
        let right = forward.cross(WORLD_UP).normalize();
        let up = right.cross(forward);
        Mat4f {
            x: vec4!(right, 0.0),
            y: vec4!(up, 0.0),
            z: vec4!(forward, 0.0),
            w: vec4!(view_ray.origin, 1.0),
        }
        .transpose()
    }

    pub(crate) fn viewport_ray(&self, viewport_position: Vec2f) -> Ray {
        let camera_origin = vec3!(0.0);
        let camera_direction = vec3!(viewport_position, 1.0).normalize();
        let camera_to_world = self.camera_to_world();
        Ray {
            origin: (camera_to_world * vec4!(camera_origin, 1.0)).xyz(),
            direction: (camera_to_world * vec4!(camera_direction, 0.0))
                .xyz()
                .normalize(),
        }
    }
}
