use std::f32::consts::FRAC_PI_4;
use std::time::Instant;

use crate::color::Color;
use crate::image::Image;
use crate::math::{Mat3, Transform, Vec2, Vec3};
use crate::primitives::Ray;

#[derive(Clone, Copy, Debug)]
pub struct CameraParams {
    /// The width of the image in pixels.
    pub image_width: u32,
    /// The height of the image in pixels.
    pub image_height: u32,
    /// The vertical field of view (FOV) in radians.
    pub fov: f32,
    /// The distance from the camera in world units of the viewing frustum’s near plane.
    // pub near: f32,
    /// The distance from the camera in world units of the viewing frustum’s far plane.
    // pub far: f32,
    /// The number of samples of single viewport pixel.
    pub samples_per_pixel: u16,
    /// The distance to the viewport in world units (also focus distance).
    pub focus_dist: f32,
    /// The variation angle of rays through each pixel in radians.
    pub defocus_angle: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    params: CameraParams,
    viewport: Viewport,
    /// Camera global transform.
    pub transform: Transform,
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            image_width: 1,
            image_height: 1,
            fov: FRAC_PI_4,
            // near: 0.1,
            // far: 1000.0,
            samples_per_pixel: 1,
            focus_dist: 1.0,
            defocus_angle: 0.0,
        }
    }
}

impl Camera {
    pub fn new(params: CameraParams) -> Self {
        Self {
            params,
            viewport: Viewport::new(
                params.fov,
                params.focus_dist,
                params.defocus_angle,
                params.image_width,
                params.image_height,
            ),
            transform: Transform::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Viewport {
    // width: f32,
    // height: f32,
    // aspect_ratio: f32,
    pixel00_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Viewport {
    fn new(
        fov: f32,
        focus_dist: f32,
        defocus_angle: f32,
        image_width: u32,
        image_height: u32,
    ) -> Self {
        let aspect_ratio = image_width as f32 / image_height as f32;
        let h = f32::tan(fov / 2.0);
        let height = 2.0 * h * focus_dist;
        let width = height * aspect_ratio;

        let pixel_delta_u = Vec3::new(width / image_width as f32, 0.0, 0.0);
        let pixel_delta_v = Vec3::new(0.0, -height / image_height as f32, 0.0);
        let pixel00_center = Vec3::new(-width / 2.0, height / 2.0, -focus_dist)
            + (pixel_delta_v + pixel_delta_u) * 0.5;

        let defocus_radius = focus_dist * f32::tan(defocus_angle / 2.0);
        let defocus_disk_u = Vec3::new(defocus_radius, 0.0, 0.0);
        let defocus_disk_v = Vec3::new(0.0, defocus_radius, 0.0);

        Self {
            // width,
            // height,
            // aspect_ratio,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_center,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    #[inline]
    fn rotated(&self, rotation: Mat3) -> Self {
        Self {
            pixel00_center: rotation * self.pixel00_center,
            pixel_delta_u: rotation * self.pixel_delta_u,
            pixel_delta_v: rotation * self.pixel_delta_v,
            defocus_disk_u: rotation * self.defocus_disk_u,
            defocus_disk_v: rotation * self.defocus_disk_v,
        }
    }

    #[inline]
    fn pixel_center(&self, x: u32, y: u32) -> Vec3 {
        self.pixel00_center + self.pixel_delta_u * x as f32 + self.pixel_delta_v * y as f32
    }
}

impl Camera {
    pub fn render_to(&self, image: &mut Image, ray_color: impl Fn(Ray) -> Color + Sync) {
        let sample_scale = f32::recip(self.params.samples_per_pixel as f32);
        let rotated_viewport = self.viewport.rotated(self.transform.rotation);

        let avail_cores = std::thread::available_parallelism().map_or(1, |n| n.get());
        let ray_color_ref = &ray_color;
        std::thread::scope(|s| {
            for mut sub_image in image.split_n(avail_cores as u32) {
                s.spawn(move || {
                    let thread_id = std::thread::current().id();
                    let y_offset = sub_image.get_y_offset();
                    println!(
                        "thread {:?} runs {}..{}",
                        thread_id,
                        y_offset,
                        y_offset + sub_image.get_height()
                    );
                    let timer = Instant::now();
                    for y in 0..sub_image.get_height() {
                        for x in 0..sub_image.get_width() {
                            let mut color = Color::BLACK;
                            for _ in 0..self.params.samples_per_pixel {
                                let ray = self.sample_ray(x, y_offset + y, &rotated_viewport);
                                color += ray_color_ref(ray);
                            }
                            color *= sample_scale;
                            sub_image.put_pixel(x, y, color);
                        }
                    }
                    let render_time = timer.elapsed();
                    println!(
                        "thread {:?} finished in {}s",
                        thread_id,
                        render_time.as_secs_f64()
                    );
                });
            }
        });
    }

    #[inline]
    fn sample_ray(&self, x: u32, y: u32, viewport: &Viewport) -> Ray {
        let offset = Vec2::random_in_square() - Vec2::splat(0.5);
        let pixel_center = viewport.pixel_center(x, y);
        let pixel_sample =
            pixel_center + viewport.pixel_delta_u * offset.x + viewport.pixel_delta_v * offset.y;

        let jittered_origin = if self.params.defocus_angle <= 0.0 {
            Vec3::ZERO
        } else {
            self.defocus_disk_sample(viewport)
        };

        let ray_origin = self.transform.translation + jittered_origin;
        let ray_direction = pixel_sample - jittered_origin;

        Ray::new(ray_origin, ray_direction)
    }

    #[inline]
    fn defocus_disk_sample(&self, viewport: &Viewport) -> Vec3 {
        let p = Vec2::random_in_disk();
        viewport.defocus_disk_u * p.x + viewport.defocus_disk_v * p.y
    }
}
