use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use num_traits::{Float, Num};

pub struct Sphere<N: Num> {
    center: Point3<N>,
    radius: N,
}

impl<N: Num> Sphere<N> {
    pub fn new(center: Point3<N>, radius: N) -> Sphere<N> {
        Sphere { center, radius }
    }
}

impl<N: Float> Hittable<N> for Sphere<N> {
    fn hit(&self, r: &Ray<N>, t_min: N, t_max: N, rec: &mut HitRecord<N>) -> bool {
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
