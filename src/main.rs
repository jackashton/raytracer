use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::objects::hittable::{Hittable, HittableList};
use raytracer::objects::{Camera, Sphere};
use raytracer::ray::Ray;
use raytracer::vec3::{Color, Point3, Vec3};
use raytracer::write::write_image;
use std::env;

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let origin = Vec3::new(4.0, 0.2, 0.0);
    let mut world: HittableList = HittableList::new();
    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5)),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - origin).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Point3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        ),
                    ));
                } else {
                    // glass
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }
    world.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
    ));
    world.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    ));
    world
}

fn color(ray_in: &Ray, world: &HittableList, depth: u32) -> Vec3<f64> {
    // stop when we exceed the max ray bounce limit
    if depth <= 0 {
        return Vec3::zero();
    }

    // t_min 0.001 to ignore hits very near to 0 to avoid shadow acne
    match world.hit(ray_in, 0.001, f64::INFINITY) {
        Some(hit) => {
            return match hit.material.scatter(ray_in, &hit) {
                Some((scattered, attenuation)) => attenuation * color(&scattered, world, depth - 1),
                _ => Vec3::zero(),
            }
        }
        _ => {}
    }

    let unit_direction = ray_in.dir.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + (Vec3::new(0.5, 0.7, 1.0) * t)
}

fn main() {
    // env vars
    dotenv().ok();
    let is_antialiasing_enabled = env::var("ANTIALIASING_ENABLED")
        .unwrap()
        .parse::<bool>()
        .unwrap();
    let antialiasing_samples_per_pixel = env::var("ANTIALIASING_SAMPLES_PER_PIXEL")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let max_depth = env::var("MAX_DEPTH").unwrap().parse::<u32>().unwrap();

    // args
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Incorrect arguments, usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = if is_antialiasing_enabled {
        antialiasing_samples_per_pixel
    } else {
        1
    };

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // Render
    let bar = ProgressBar::new(image_width as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed} elapsed] {wide_bar} {percent}% [{eta} remaining] rendering"),
    );

    let scene: Vec<Vec<Color>> = (0..image_width)
        .into_par_iter()
        .map(|i| {
            let mut rng = rand::thread_rng();
            let col = (0..image_height)
                .rev()
                .map(|j| {
                    let mut pixel_color = Vec3::zero();
                    for _s in 0..samples_per_pixel {
                        let (u_ran, v_ran) = if is_antialiasing_enabled {
                            (rng.gen(), rng.gen())
                        } else {
                            (0.0, 0.0)
                        };
                        let u = ((i as f64) + u_ran) / (image_width as f64 - 1.0);
                        let v = ((j as f64) + v_ran) / (image_height as f64 - 1.0);
                        let ray = cam.get_ray(u, v);
                        pixel_color += color(&ray, &world, max_depth);
                    }
                    // divide color by number of samples per pixel and gamma correct for gamma 2
                    let scale = 1.0 / (samples_per_pixel as f64);
                    Color::from((pixel_color * scale).sqrt().clamp(0.0, 0.999) * 256.0)
                })
                .collect();
            bar.inc(1);
            col
        })
        .collect();

    bar.finish();

    write_image(scene, filename)
}
