use indicatif::{ProgressBar, ProgressStyle};
use raytracer::ray::Ray;
use raytracer::vec3::{Color, Point3, Vec3};
use raytracer::write::write_image;
use std::env;

fn hit_sphere(center: &Point3<f64>, radius: f64, r: &Ray<f64>) -> bool {
    let oc = r.orig - *center;
    let a = Vec3::dot(&r.dir, &r.dir);
    let b = 2.0 * Vec3::dot(&oc, &r.dir);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(r: &Ray<f64>) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        return Color::new(255, 0, 0);
    }

    let unit_direction = Vec3::unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    let v = (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t);
    <Color>::from(v * 255.999)
}

fn main() {
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

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3<f64> = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3<f64> = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

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
                    let u = i as f64 / (image_width as f64 - 1.0);
                    let v = j as f64 / (image_height as f64 - 1.0);
                    let r = Ray::new(
                        origin,
                        lower_left_corner + (horizontal * u) + (vertical * v) - origin,
                    );
                    color(&r)
                })
                .collect();
            bar.inc(1);
            col
        })
        .collect();

    bar.finish();

    write_image(scene, filename)
}
