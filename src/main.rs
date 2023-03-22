use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::objects::hittable::{Hittable, HittableList};
use raytracer::objects::{Camera, Sphere};
use raytracer::ray::Ray;
use raytracer::vec3::{Color, Point3, Vec3};
use raytracer::write::write_image;
use std::env;

fn color(r: &Ray, world: &HittableList<dyn Hittable>, depth: u32) -> Vec3<f64> {
    // stop when we exceed the max ray bounce limit
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // t_min 0.001 to ignore hits very near to 0 to avoid shadow acne
    match world.hit(r, 0.001, f64::INFINITY) {
        Some(rec) => {
            return match rec.material.scatter(r, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * color(&scattered, world, depth - 1) * 0.5
                }
                _ => Vec3::new(0.0, 0.0, 0.0),
            }
        }
        _ => {}
    }

    let unit_direction = Vec3::unit_vector(r.dir);
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
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = if is_antialiasing_enabled {
        antialiasing_samples_per_pixel
    } else {
        1
    };

    // World
    let mut world: HittableList<dyn Hittable> = HittableList::new();

    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Dielectric::new(1.5);
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let cam = Camera::new();

    // Misc
    let mut rng = rand::thread_rng();

    // Render
    let bar = ProgressBar::new(image_width as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed} elapsed] {wide_bar} {percent}% [{eta} remaining] rendering"),
    );

    let scene: Vec<Vec<Color>> = (0..image_width)
        .map(|i| {
            let col = (0..image_height)
                .rev()
                .map(|j| {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let (u_ran, v_ran) = if is_antialiasing_enabled {
                            (rng.gen(), rng.gen())
                        } else {
                            (0.0, 0.0)
                        };
                        let u = ((i as f64) + u_ran) / (image_width as f64 - 1.0);
                        let v = ((j as f64) + v_ran) / (image_height as f64 - 1.0);
                        let r = cam.get_ray(u, v);
                        pixel_color += color(&r, &world, max_depth);
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
