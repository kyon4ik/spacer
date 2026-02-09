use std::ops::Mul;

use crate::math::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Mat3::from_diagonal(1.0);

    pub const fn from_diagonal(value: f32) -> Self {
        Self {
            x_axis: Vec3::new(value, 0.0, 0.0),
            y_axis: Vec3::new(0.0, value, 0.0),
            z_axis: Vec3::new(0.0, 0.0, value),
        }
    }

    pub const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut res = self.x_axis.mul(rhs.x);
        res += self.y_axis.mul(rhs.y);
        res += self.z_axis.mul(rhs.z);
        res
    }
}
