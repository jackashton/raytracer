use crate::vec3::{Point3, Vec3};
use rand::Rng;
use std::f64::consts::PI;

pub fn random_in_unit_sphere() -> Point3<f64> {
    let mut rng = rand::thread_rng();
    let u: f64 = rng.gen();
    let p = Point3::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    );
    p.normalize() * u.cbrt()
}

pub fn random_unit_vector() -> Vec3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_in_hemisphere(normal: &Vec3<f64>) -> Vec3<f64> {
    let in_unit_sphere: Vec3<f64> = random_in_unit_sphere();
    if in_unit_sphere.dot(&normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Point3<f64> {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.0..1.0);
    let theta = rng.gen_range(0.0..2.0 * PI);
    Point3 {
        x: r * theta.cos(),
        y: r * theta.sin(),
        z: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_in_unit_sphere_in_expected_range() {
        let p = random_in_unit_sphere();
        let range = -1.0..1.0;
        assert!(range.contains(&p.x));
        assert!(range.contains(&p.y));
        assert!(range.contains(&p.z));
    }

    #[test]
    fn test_random_unit_vector_is_unit_vector() {
        let v = random_unit_vector();
        assert_eq!(v.length(), 1.0);
    }

    #[test]
    fn test_random_in_hemisphere_with_positive_dot_product() {
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let v = random_in_hemisphere(&normal);
        assert!(v.y > 0.0);
    }

    #[test]
    fn test_random_in_hemisphere_with_negative_dot_product() {
        let normal = Vec3::new(0.0, -1.0, 0.0);
        let v = random_in_hemisphere(&normal);
        assert!(v.y < 0.0);
    }

    #[test]
    fn test_random_in_unit_disk() {
        let point = random_in_unit_disk();

        // Ensure the point is within the unit disk
        assert!(point.length() < 1.0);

        // Ensure the point is on the xy plane
        assert_eq!(point.z, 0.0);
    }
}
