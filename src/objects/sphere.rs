use crate::material::Material;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere<M: Material> {
    center: Point3<f64>,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3<f64>, radius: f64, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length() * r.dir.length();
        let h = Vec3::dot(&oc, &r.dir);
        let c = oc.length() * oc.length() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-h - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-h + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        // normal always points against the incident ray
        let mut normal = (p - self.center) / self.radius;
        let mut front_face = true;
        if Vec3::dot(&r.dir, &normal).is_sign_positive() {
            normal = -normal;
            front_face = false;
        }
        Some(HitRecord {
            t,
            p,
            normal,
            front_face,
            material: &self.material,
        })
    }
}

// TODO better tests
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_hit() {
    //     let center = Point3::new(0.0, 0.0, -1.0);
    //     let sphere = Sphere::new(center, 0.5);
    //     let origin = Point3::new(0.0, 0.0, 0.0);
    //     let ray = Ray::new(origin, center);
    //     assert!(sphere.hit(&ray, 0.0, f64::INFINITY).is_some())
    // }
}
