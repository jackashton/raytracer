use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3<f64>,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    lower_left_corner: Point3<f64>,
}

impl Camera {
    pub fn new(
        origin: Point3<f64>,
        target: Point3<f64>,
        vup: Vec3<f64>,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - target).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        )
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
