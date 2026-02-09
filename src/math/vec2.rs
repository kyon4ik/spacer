use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::splat(0.0);
    pub const ONE: Vec2 = Vec2::splat(1.0);
    pub const X: Vec2 = Vec2::new(1.0, 0.0);
    pub const Y: Vec2 = Vec2::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn splat(v: f32) -> Self {
        Self::new(v, v)
    }

    /// Returns random vector in unit square
    #[inline]
    pub fn random_in_square() -> Self {
        Self::new(fastrand::f32(), fastrand::f32())
    }

    /// Returns random vector in unit disk
    #[inline]
    pub fn random_in_disk() -> Self {
        loop {
            let v = Self::random_in_square() * 2.0 - Vec2::ONE;
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// In 2D, the cross product is a scalar representing the signed area
    /// of the parallelogram formed by the two vectors.
    #[inline]
    pub fn cross(&self, rhs: &Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let length = self.length();
        debug_assert!(length.is_finite() && length > 0.0);
        *self / length
    }
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Self::Output {
        vec2(-self.x, -self.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        vec2(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.mul(rhs.recip())
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.mul_assign(rhs.recip())
    }
}
