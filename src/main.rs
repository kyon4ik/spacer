use std::rc::Rc;
use std::time::Instant;

use spacer::camera::{Camera, CameraParams};
use spacer::color::Color;
use spacer::image::Image;
use spacer::material::{Material, MaterialKind, StandardMaterial};
use spacer::math::{Transform, Vec3, vec3};
use spacer::primitives::{Hittable, HittableList, Ray, Sphere, SphereList};

const SKY_COLOR: Color = Color::new(0.5, 0.7, 1.0);

fn main() {
    fastrand::seed(8767162531530871546);
    println!("Random seed: {}", fastrand::get_seed());

    let mut image = Image::from_aspect_ratio(1200, 16.0 / 9.0);

    let camera_params = CameraParams {
        image_width: image.get_width(),
        image_height: image.get_height(),
        fov: f32::to_radians(20.0),
        samples_per_pixel: 1,
        defocus_angle: f32::to_radians(0.6),
        focus_dist: 10.0,
    };

    let mut camera = Camera::new(camera_params);
    camera.transform = Transform::look_at(vec3(13.0, 2.0, 3.0), Vec3::ZERO, Vec3::Y);

    let world = final_world();

    println!(
        "Image resolution: {}x{}",
        image.get_width(),
        image.get_height()
    );

    let timer = Instant::now();
    camera.render_to(&mut image, |ray| ray_color(ray, &world, 50));
    let render_time = timer.elapsed();
    println!("Render in: {:.6}s", render_time.as_secs_f64());

    let image_path = "output/image.ppm";
    image.save_as_ppm(image_path).unwrap();
    println!("Image saved to {}", image_path);
}

fn ray_color(ray: Ray, world: &impl Hittable, bounces: u8) -> Color {
    if bounces == 0 {
        return Color::BLACK;
    }

    if let Some(hit) = ray.hit(world, 0.001..f32::INFINITY) {
        if let Some((attenuation, scattered_ray)) = hit.material.scatter(&ray, &hit) {
            return ray_color(scattered_ray, world, bounces - 1) * attenuation;
        }

        return Color::BLACK;
    }

    let ray_norm_dir = ray.direction().normalized();
    let a = 0.5 * (ray_norm_dir.y + 1.0);
    debug_assert!((0.0..=1.0).contains(&a));
    Color::WHITE * (1.0 - a) + SKY_COLOR * a
}

fn final_world() -> SphereList {
    let mut world = SphereList::default();
    let ground_material = StandardMaterial::from_color(Color::new(0.5, 0.5, 0.5));
    world.add(Rc::new(Sphere {
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
                    StandardMaterial::from_color(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Color::random() * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = fastrand::f32() * 0.5;
                    StandardMaterial {
                        kind: MaterialKind::Metalic,
                        albedo,
                        fuzz,
                        ..Default::default()
                    }
                } else {
                    StandardMaterial {
                        kind: MaterialKind::Dielectric,
                        ior: 1.5,
                        ..Default::default()
                    }
                };

                world.add(Rc::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }));
            }
        }
    }

    let material1 = StandardMaterial {
        kind: MaterialKind::Dielectric,
        ior: 1.5,
        ..Default::default()
    };
    world.add(Rc::new(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));

    let material2 = StandardMaterial::from_color(Color::new(0.4, 0.2, 0.1));
    world.add(Rc::new(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));

    let material3 = StandardMaterial {
        kind: MaterialKind::Metalic,
        albedo: Color::new(0.7, 0.6, 0.5),
        ..Default::default()
    };
    world.add(Rc::new(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    world
}

fn test_world() -> HittableList {
    let material_ground = StandardMaterial {
        albedo: Color::new(0.8, 0.8, 0.0),
        ..Default::default()
    };
    let material_center = StandardMaterial {
        albedo: Color::new(0.1, 0.2, 0.5),
        ..Default::default()
    };
    let material_left = StandardMaterial {
        kind: MaterialKind::Dielectric,
        ior: 1.5,
        ..Default::default()
    };
    let material_bubble = StandardMaterial {
        kind: MaterialKind::Dielectric,
        ior: 1.0 / 1.5,
        ..Default::default()
    };
    let material_right = StandardMaterial {
        kind: MaterialKind::Metalic,
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
        ..Default::default()
    };

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere {
        center: vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    }));
    world.add(Rc::new(Sphere {
        center: vec3(0.0, 0.0, -1.2),
        radius: 0.5,
        material: material_center,
    }));
    world.add(Rc::new(Sphere {
        center: vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left,
    }));
    world.add(Rc::new(Sphere {
        center: vec3(-1.0, 0.0, -1.0),
        radius: 0.4,
        material: material_bubble,
    }));
    world.add(Rc::new(Sphere {
        center: vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right,
    }));

    world
}
