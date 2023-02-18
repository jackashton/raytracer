use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use num_traits::{Float, Num};

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl<N: Float> Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: N, t_max: N, rec: &mut HitRecord) -> bool {
        let oc: Point3<N> = r.orig - self.center;
        let a: N = r.dir.length() * r.dir.length();
        let h = Vec3::dot(&oc, &r.dir);
        let c: N = oc.length() * oc.length() - self.radius * self.radius;

        let discriminant: N = h * h - a * c;
        if discriminant.is_sign_negative() {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (-h - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-h + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        // normal always points against the incident ray
        rec.normal = (rec.p - self.center) / self.radius;
        if Vec3::dot(&r.dir, &rec.normal).is_sign_positive() {
            rec.normal = -rec.normal;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit() {
        let center = Point3::new(0.0, 0.0, -1.0);
        let sphere = Sphere::new(center, 0.5);
        let origin = Point3::new(0.0, 0.0, 0.0);
        let ray = Ray::new(origin, center);
        let mut rec: HitRecord = HitRecord::new();
        assert!(sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec))
    }
}
