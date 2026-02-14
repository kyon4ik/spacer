use std::sync::Arc;
use std::time::Instant;

use spacer::camera::{Camera, CameraParams};
use spacer::color::Color;
use spacer::image::Image;
use spacer::material::Material;
use spacer::math::{Interval, Vec3};
use spacer::primitives::{Hittable, HittableList, Ray, Sphere};
use spacer::renderer::{Renderer, StRenderer};

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 450;

fn main() {
    let mut image = Image::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    let camera_params = CameraParams {
        image_width: CANVAS_WIDTH,
        image_height: CANVAS_HEIGHT,
        fov: f32::to_radians(90.0),
        ..Default::default()
    };
    let camera = Camera::new(camera_params);
    log::info!("Aspect ratio: {}", camera.aspect_ratio());

    let mut world = HittableList::default();
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::lambertian(Color::RED),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::lambertian(Color::GREEN),
    }));

    let render_timer = Instant::now();
    let renderer = StRenderer;
    renderer.render(&camera, &mut image, |ray| {
        ray_color(&world, ray, Interval::new(0.0, f32::INFINITY))
    });

    let frame_time = render_timer.elapsed();
    println!("Frame rendered in {}ms", frame_time.as_millis());

    image
        .save_as_ppm("output/raytracer.ppm")
        .expect("Saving image");
}

fn ray_color(world: &impl Hittable, ray: Ray, t_range: Interval) -> Color {
    if let Some(hit) = world.hit(&ray, t_range) {
        return Color::from((hit.normal + Vec3::ONE) * 0.5);
    }

    let a = 0.5 * (ray.direction().y + 1.0);
    Color::from(Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a)
}
