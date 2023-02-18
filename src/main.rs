use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use raytracer::objects::hittable::{HitRecord, Hittable, HittableList};
use raytracer::objects::{Camera, Sphere};
use raytracer::ray::Ray;
use raytracer::vec3::{Color, Point3, Vec3};
use raytracer::write::write_image;
use std::env;

fn color(r: &Ray, world: &HittableList<dyn Hittable>) -> Vec3<f64> {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return (rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
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
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
                        pixel_color += color(&r, &world);
                    }
                    let scale = 1.0 / (samples_per_pixel as f64);
                    pixel_color *= scale;
                    Color::from(pixel_color.clamp(0.0, 0.999) * 256.0)
                })
                .collect();
            bar.inc(1);
            col
        })
        .collect();

    bar.finish();

    write_image(scene, filename)
}
