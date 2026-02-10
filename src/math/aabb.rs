use crate::math::{Interval, Vec3};
use crate::primitives::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub x_axis: Interval,
    pub y_axis: Interval,
    pub z_axis: Interval,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl Default for Aabb {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Aabb {
    pub const EMPTY: Aabb = Self::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY);

    pub const fn new(x_axis: Interval, y_axis: Interval, z_axis: Interval) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    pub const fn from_center(center: Vec3, half_size: Vec3) -> Self {
        Self::new(
            Interval::new(center.x - half_size.x, center.x + half_size.x),
            Interval::new(center.y - half_size.y, center.y + half_size.y),
            Interval::new(center.z - half_size.z, center.z + half_size.z),
        )
    }

    pub const fn from_corners(a: Vec3, b: Vec3) -> Self {
        Self::new(
            Interval::ordered(a.x, b.x),
            Interval::ordered(a.y, b.y),
            Interval::ordered(a.z, b.z),
        )
    }

    pub fn enclose(self, other: Self) -> Self {
        Self::new(
            self.x_axis.enclose(other.x_axis),
            self.y_axis.enclose(other.y_axis),
            self.z_axis.enclose(other.z_axis),
        )
    }

    pub fn longest_axis(&self) -> Axis {
        if self.x_axis.length() > self.y_axis.length() {
            if self.x_axis.length() > self.z_axis.length() {
                Axis::X
            } else {
                Axis::Z
            }
        } else if self.y_axis.length() > self.z_axis.length() {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        let ray_origin = ray.origin();
        let ray_dir = ray.direction();

        let axes = [
            (self.x_axis, ray_origin.x, ray_dir.x),
            (self.y_axis, ray_origin.y, ray_dir.y),
            (self.z_axis, ray_origin.z, ray_dir.z),
        ];

        let mut intersection = ray_t;
        for (axis, origin, dir) in axes {
            let t0 = (axis.min - origin) / dir;
            let t1 = (axis.max - origin) / dir;

            let axis_int = Interval::ordered(t0, t1);
            intersection = intersection.intersect(axis_int);

            if intersection.is_empty() {
                return false;
            }
        }

        true
    }
}
