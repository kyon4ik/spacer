use std::ops::Range;
use std::rc::Rc;

use crate::material::Material;
use crate::math::Vec3;

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

    pub fn hit(&self, object: &impl Hittable, t_range: Range<f32>) -> Option<HitRecord> {
        object.hit(self, t_range)
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

#[derive(Default)]
pub struct SphereList {
    spheres: Vec<Sphere>,
}

impl SphereList {
    pub fn add(&mut self, object: Rc<Sphere>) {
        self.spheres.push(*object);
    }
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray, t_range.clone()))
            .min_by(|hit1, hit2| hit1.t.total_cmp(&hit2.t))
    }
}

impl Hittable for SphereList {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        self.spheres
            .iter()
            .filter_map(|object| object.hit(ray, t_range.clone()))
            .min_by(|hit1, hit2| hit1.t.total_cmp(&hit2.t))
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
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
        if !t_range.contains(&t) {
            t = (h + dsqrt) / a;
            if !t_range.contains(&t) {
                return None;
            }
        }

        let point = ray.at(t);
        let out_normal = (point - self.center) / self.radius;
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
}
