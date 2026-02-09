use crate::math::{Mat3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub rotation: Mat3,
    pub translation: Vec3,
}

impl Transform {
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            rotation: Mat3::IDENTITY,
            translation,
        }
    }

    #[inline]
    pub fn look_to(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        let front = dir.normalized();
        let right = front.cross(&up).normalized();
        let up = right.cross(&front);

        Self {
            rotation: Mat3::from_cols(right, up, -front),
            translation: eye,
        }
    }

    #[inline]
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self::look_to(eye, center - eye, up)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: Mat3::IDENTITY,
            translation: Vec3::ZERO,
        }
    }
}
