use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::math::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(Vec3);

impl Color {
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub const RED: Color = Color::new(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn random() -> Self {
        Self(Vec3::random_in_cube())
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.0.x
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.0.y
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.0.z
    }

    #[inline]
    pub fn linear_to_gamma(&self) -> Color {
        Color::new(
            self.r().clamp(0.0, 1.0).sqrt(),
            self.g().clamp(0.0, 1.0).sqrt(),
            self.b().clamp(0.0, 1.0).sqrt(),
        )
    }
}

impl Add for Color {
    type Output = Color;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0)
    }
}

impl AddAssign for Color {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Color(self.0 * rhs)
    }
}

impl MulAssign<f32> for Color {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}
