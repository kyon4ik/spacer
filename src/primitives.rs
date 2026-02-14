use std::sync::Arc;

use crate::material::Material;
use crate::math::{Aabb, Axis, Interval, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;
}

pub struct BvhNode {
    left: Arc<dyn Hittable + Sync + Send>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &mut HittableList) -> Self {
        Self::from_hittables(list.objects.as_mut_slice())
    }

    fn from_hittables(objects: &mut [Arc<dyn Hittable + Send + Sync>]) -> Self {
        let mut bbox = Aabb::EMPTY;
        for object in objects.iter() {
            bbox = bbox.enclose(object.bounding_box());
        }

        let compare = match bbox.longest_axis() {
            Axis::X => |v: &Aabb, u: &Aabb| v.x_axis.cmp_min(&u.x_axis),
            Axis::Y => |v: &Aabb, u: &Aabb| v.y_axis.cmp_min(&u.y_axis),
            Axis::Z => |v: &Aabb, u: &Aabb| v.z_axis.cmp_min(&u.z_axis),
        };

        let children: (
            Arc<dyn Hittable + Send + Sync>,
            Arc<dyn Hittable + Send + Sync>,
        ) = if objects.len() == 1 {
            (objects[0].clone(), objects[0].clone())
        } else if objects.len() == 2 {
            (objects[0].clone(), objects[1].clone())
        } else {
            objects.sort_unstable_by(|a, b| compare(&a.bounding_box(), &b.bounding_box()));

            let midpoint = objects.len() / 2;
            let (left, right) = objects.split_at_mut(midpoint);
            (
                Arc::new(Self::from_hittables(left)),
                Arc::new(Self::from_hittables(right)),
            )
        };

        let (left, right) = children;
        Self { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_range) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_range);

        let right_t_max = hit_left.map_or(t_range.max, |hit| hit.t);
        if let Some(hit_right) = self.right.hit(ray, Interval::new(t_range.min, right_t_max)) {
            Some(hit_right)
        } else {
            hit_left
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
    pub material: Material,
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn hit(&self, object: &impl Hittable, t_range: Interval) -> Option<HitRecord> {
        object.hit(self, t_range)
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.bbox = self.bbox.enclose(object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray, t_range))
            .min_by(|hit1, hit2| hit1.t.total_cmp(&hit2.t))
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();

        let a = ray.direction().length_squared();
        // h = -b / 2
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let dsqrt = discriminant.sqrt();
        let mut t = (h - dsqrt) / a;
        if !t_range.contains(t) {
            t = (h + dsqrt) / a;
            if !t_range.contains(t) {
                return None;
            }
        }

        let point = ray.at(t);
        let out_normal = (point - self.center) / self.radius;
        // This can be slightly of due to floating errors
        let out_normal = out_normal.fast_renormalized();

        let is_front_face = ray.direction().dot(&out_normal) < 0.0;
        let normal = if is_front_face {
            out_normal
        } else {
            -out_normal
        };
        Some(HitRecord {
            point,
            normal,
            t,
            is_front_face,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::from_center(self.center, Vec3::splat(self.radius))
    }
}
