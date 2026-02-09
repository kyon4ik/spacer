use crate::color::Color;
use crate::math::Vec3;
use crate::primitives::{HitRecord, Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Clone, Copy, Debug, Default)]
pub enum MaterialKind {
    #[default]
    Lambertian,
    Metalic,
    Dielectric,
}

#[derive(Clone, Copy, Debug)]
pub struct StandardMaterial {
    pub kind: MaterialKind,
    pub albedo: Color,
    /// Fuzziness of the metalic material
    pub fuzz: f32,
    /// Index of refraction relative to the environment
    pub ior: f32,
}

impl StandardMaterial {
    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: color,
            ..Default::default()
        }
    }
}

impl Default for StandardMaterial {
    fn default() -> Self {
        Self {
            kind: MaterialKind::default(),
            albedo: Color::WHITE,
            fuzz: 0.0,
            ior: 1.5,
        }
    }
}

impl Material for StandardMaterial {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        match self.kind {
            MaterialKind::Lambertian => {
                let mut scatter_dir = hit.normal + Vec3::random_on_sphere();
                if scatter_dir.relative_eq(&Vec3::ZERO) {
                    scatter_dir = hit.normal;
                }
                let scattered_ray = Ray::new(hit.point, scatter_dir);
                Some((self.albedo, scattered_ray))
            }
            MaterialKind::Metalic => {
                let reflect_dir = ray.direction().reflect(&hit.normal);
                let fuzzed_dir = reflect_dir.normalized() + (Vec3::random_on_sphere() * self.fuzz);
                let scattered_ray = Ray::new(hit.point, fuzzed_dir);

                if scattered_ray.direction().dot(&hit.normal) > 0.0 {
                    Some((self.albedo, scattered_ray))
                } else {
                    None
                }
            }
            MaterialKind::Dielectric => {
                let ior = if hit.is_front_face {
                    self.ior.recip()
                } else {
                    self.ior
                };

                let ray_dir = ray.direction().normalized();
                let mut refracted_dir = ray_dir.refract(&hit.normal, ior);
                let cos_theta = f32::min(-ray_dir.dot(&hit.normal), 1.0);

                // Can not refract - total internal reflection
                if refracted_dir == Vec3::ZERO || reflectance(cos_theta, ior) > fastrand::f32() {
                    refracted_dir = ray_dir.reflect(&hit.normal);
                }

                let scattered_ray = Ray::new(hit.point, refracted_dir);
                Some((Color::WHITE, scattered_ray))
            }
        }
    }
}

// Schlick's approximation
fn reflectance(cosine: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}
