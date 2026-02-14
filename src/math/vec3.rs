use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::math::Vec2;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(Vec2, f32)> for Vec3 {
    fn from((vec, z): (Vec2, f32)) -> Self {
        vec3(vec.x, vec.y, z)
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::splat(0.0);
    pub const ONE: Vec3 = Vec3::splat(1.0);
    pub const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(v: f32) -> Self {
        Self::new(v, v, v)
    }

    /// Returns random vector in unit cube
    #[inline]
    pub fn random_in_cube() -> Self {
        Self::new(fastrand::f32(), fastrand::f32(), fastrand::f32())
    }

    /// Returns random unit vector on unit sphere
    #[inline]
    pub fn random_on_sphere() -> Self {
        loop {
            let v = Self::random_in_cube() * 2.0 - Vec3::ONE;
            let len2 = v.length_squared();
            if 1e-20 < len2 && len2 <= 1.0 {
                return v / len2.sqrt();
            }
        }
    }

    /// Returns random vector on a hemisphere with the given `normal`
    #[inline]
    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let v = Self::random_on_sphere();
        if v.dot(normal) > 0.0 { v } else { -v }
    }

    #[inline]
    pub fn relative_eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < 1e-6
            && f32::abs(self.y - other.y) < 1e-6
            && f32::abs(self.z - other.z) < 1e-6
    }

    #[inline]
    pub fn reflect(&self, normal: &Self) -> Self {
        debug_assert!(normal.is_normalized());
        *self - *normal * 2.0 * normal.dot(self)
    }

    #[inline]
    pub fn refract(&self, normal: &Self, eta: f32) -> Self {
        debug_assert!(normal.is_normalized());
        debug_assert!(self.is_normalized());
        let n_dot_i = self.dot(normal);
        let k = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
        if k >= 0.0 {
            *self * eta - *normal * (eta * n_dot_i + k.sqrt())
        } else {
            Self::ZERO
        }
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let length = self.length();
        debug_assert!(length.is_finite() && length > 0.0);
        self.div(length)
    }

    #[inline]
    pub fn fast_renormalized(&self) -> Self {
        let length_squared = self.length_squared();
        self.mul(0.5 * (3.0 - length_squared))
    }

    #[inline]
    pub fn is_normalized(&self) -> bool {
        f32::abs(self.length() - 1.0) <= 2e-4
    }

    #[inline]
    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        debug_assert!((0.0..=1.0).contains(&t));
        self * (1.0 - t) + rhs * t
    }
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        vec3(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        vec3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        vec3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        vec3(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        vec3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.mul(rhs.recip())
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.mul_assign(rhs.recip())
    }
}
