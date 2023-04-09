use crate::ray::Ray;
use crate::vec3::utils::random_in_unit_disk;
use crate::vec3::{Point3, Vec3};
use rand::Rng;

pub struct Camera {
    origin: Point3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    lower_left_corner: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    lens_radius: f64,
    time0: f64, // shutter open time
    time1: f64, // shutter close time
}

impl Camera {
    pub fn new(
        origin: Point3<f64>,
        target: Point3<f64>,
        vup: Vec3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - target).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let mut rng = rand::thread_rng();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
                - self.origin
                - offset,
            rng.gen_range(self.time0..self.time1),
        )
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
