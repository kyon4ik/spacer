use crate::color::Color;
use crate::math::Vec3;
use crate::primitives::{HitRecord, Ray};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(LambertianMaterial),
    Metalic(MetalicMaterial),
    Dielectric(DielectricMaterial),
}

#[derive(Clone, Copy, Debug)]
pub struct LambertianMaterial {
    pub albedo: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct MetalicMaterial {
    pub albedo: Color,
    /// Fuzziness of the metalic material
    pub fuzz: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct DielectricMaterial {
    /// Index of refraction relative to the environment
    pub ior: f32,
}

impl Material {
    pub const fn lambertian(albedo: Color) -> Self {
        Self::Lambertian(LambertianMaterial { albedo })
    }

    pub const fn metalic(albedo: Color, fuzz: f32) -> Self {
        Self::Metalic(MetalicMaterial { albedo, fuzz })
    }

    pub const fn dielectric(ior: f32) -> Self {
        Self::Dielectric(DielectricMaterial { ior })
    }

    pub fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian(mat) => {
                let mut scatter_dir = hit.normal + Vec3::random_on_sphere();
                if scatter_dir.relative_eq(&Vec3::ZERO) {
                    scatter_dir = hit.normal;
                }
                let scattered_ray = Ray::new(hit.point, scatter_dir);
                Some((mat.albedo, scattered_ray))
            }
            Self::Metalic(mat) => {
                let reflect_dir = ray.direction().reflect(&hit.normal);
                let fuzzed_dir = reflect_dir.normalized() + (Vec3::random_on_sphere() * mat.fuzz);
                let scattered_ray = Ray::new(hit.point, fuzzed_dir);

                if scattered_ray.direction().dot(&hit.normal) > 0.0 {
                    Some((mat.albedo, scattered_ray))
                } else {
                    None
                }
            }
            Self::Dielectric(mat) => {
                let ior = if hit.is_front_face {
                    mat.ior.recip()
                } else {
                    mat.ior
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

impl Default for LambertianMaterial {
    fn default() -> Self {
        Self {
            albedo: Color::WHITE,
        }
    }
}

impl Default for MetalicMaterial {
    fn default() -> Self {
        Self {
            albedo: Color::WHITE,
            fuzz: 0.0,
        }
    }
}

impl Default for DielectricMaterial {
    fn default() -> Self {
        Self { ior: 1.5 }
    }
}

// Schlick's approximation
fn reflectance(cosine: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}
