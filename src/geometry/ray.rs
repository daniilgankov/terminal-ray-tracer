use crate::math::vec3::Vec3f;

#[derive(Clone, Copy)]
pub(crate) struct Ray {
    pub(crate) origin: Vec3f,
    pub(crate) direction: Vec3f,
}
