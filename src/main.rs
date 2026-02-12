use std::sync::Arc;
use std::time::Instant;

use spacer::camera::{Camera, CameraParams};
use spacer::color::Color;
use spacer::image::{Image, RenderTarget};
use spacer::material::Material;
use spacer::math::{Interval, Transform, Vec3, vec3};
use spacer::primitives::{BvhNode, Hittable, HittableList, Ray, Sphere};
use spacer::renderer::{MtRenderer, Renderer, StRenderer};

const SKY_COLOR: Color = Color::new(0.5, 0.7, 1.0);

fn main() {
    init_logger(log::LevelFilter::Debug);
    fastrand::seed(8767162531530871546);
    log::info!("Random seed: {}", fastrand::get_seed());

    let mut image = Image::from_aspect_ratio(600, 16.0 / 9.0);

    let camera_params = CameraParams {
        image_width: image.get_width(),
        image_height: image.get_height(),
        fov: f32::to_radians(20.0),
        samples_per_pixel: 32,
        defocus_angle: f32::to_radians(0.6),
        focus_dist: 10.0,
    };

    let mut camera = Camera::new(camera_params);
    camera.transform = Transform::look_at(vec3(13.0, 2.0, 3.0), Vec3::ZERO, Vec3::Y);

    let world = final_world();

    log::info!(
        "Image resolution: {}x{}",
        image.get_width(),
        image.get_height()
    );

    let timer = Instant::now();
    let renderer = MtRenderer::default();
    renderer.render(&camera, &mut image, |ray| ray_color(ray, &world, 50));
    let render_time = timer.elapsed();
    log::info!("Render in: {:.6}s", render_time.as_secs_f64());

    let image_path = "output/image.ppm";
    image.save_as_ppm(image_path).unwrap();
    log::info!("Image saved to {}", image_path);
}

fn ray_color(ray: Ray, world: &impl Hittable, bounces: u8) -> Color {
    if bounces == 0 {
        return Color::BLACK;
    }

    if let Some(hit) = ray.hit(world, Interval::new(0.001, f32::INFINITY)) {
        if let Some((attenuation, scattered_ray)) = hit.material.scatter(&ray, &hit) {
            return ray_color(scattered_ray, world, bounces - 1) * attenuation;
        }

        return Color::BLACK;
    }

    let ray_norm_dir = ray.direction().normalized();
    let a = 0.5 * (ray_norm_dir.y + 1.0);
    Color::WHITE.lerp(SKY_COLOR, a)
}

fn final_world() -> impl Hittable {
    let mut world = HittableList::default();
    let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Arc::new(Sphere {
        center: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fastrand::f64();
            let center = vec3(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Material::lambertian(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Color::random() * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = fastrand::f32() * 0.5;
                    Material::metalic(albedo, fuzz)
                } else {
                    Material::dielectric(1.5)
                };

                world.add(Arc::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }));
            }
        }
    }

    let material1 = Material::dielectric(1.5);
    world.add(Arc::new(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));

    let material2 = Material::lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));

    let material3 = Material::metalic(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    BvhNode::new(&mut world)
}

struct SimpleLogger {
    filter: std::sync::OnceLock<log::LevelFilter>,
}
static LOGGER: SimpleLogger = SimpleLogger {
    filter: std::sync::OnceLock::new(),
};

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level().to_level_filter() <= *self.filter.get_or_init(|| log::LevelFilter::Off)
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(level: log::LevelFilter) {
    LOGGER.filter.set(level).unwrap();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level);
}
