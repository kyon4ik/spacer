use std::time::Instant;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::{Image, RenderTarget};
use crate::primitives::Ray;

pub trait Renderer {
    fn render<F>(&self, camera: &Camera, image: &mut Image, ray_color: F)
    where
        F: Fn(Ray) -> Color + Sync;
}

#[derive(Default)]
pub struct StRenderer;

pub struct MtRenderer {
    n_workers: usize,
}

impl MtRenderer {
    pub fn new(n_workers: usize) -> Self {
        Self { n_workers }
    }
}

impl Default for MtRenderer {
    fn default() -> Self {
        let n_workers = std::thread::available_parallelism().map_or(1, |n| n.get());
        Self { n_workers }
    }
}

impl Renderer for StRenderer {
    fn render<F>(&self, camera: &Camera, image: &mut Image, ray_color: F)
    where
        F: Fn(Ray) -> Color + Sync,
    {
        camera.render_to(image, ray_color);
    }
}

impl Renderer for MtRenderer {
    fn render<F>(&self, camera: &Camera, image: &mut Image, ray_color: F)
    where
        F: Fn(Ray) -> Color + Sync,
    {
        let ray_color_ref = &ray_color;
        std::thread::scope(|s| {
            for mut sub_image in image.split_n(self.n_workers as u32) {
                s.spawn(move || {
                    let thread_id = std::thread::current().id();
                    let y_offset = sub_image.get_y_offset();
                    log::debug!(
                        "thread {:?} runs {}..{}",
                        thread_id,
                        y_offset,
                        y_offset + sub_image.get_height()
                    );

                    let timer = Instant::now();
                    camera.render_to(&mut sub_image, ray_color_ref);

                    let render_time = timer.elapsed();
                    log::debug!(
                        "thread {:?} finished in {}s",
                        thread_id,
                        render_time.as_secs_f64()
                    );
                });
            }
        });
    }
}
