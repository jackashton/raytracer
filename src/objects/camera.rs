use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use num_traits::Num;

pub struct Camera<N: Num> {
    origin: Point3<N>,
    horizontal: Vec3<N>,
    vertical: Vec3<N>,
    lower_left_corner: Point3<N>,
}

impl Camera<f64> {
    pub fn new() -> Camera<f64> {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3<f64> = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Vec3<f64> = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray<f64> {
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
