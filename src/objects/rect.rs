use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use num_traits::Num;

pub struct Rect<N: Num> {
    a: Point3<N>,
    b: Point3<N>,
}

impl<N: Num> Rect<N> {
    pub fn new(a: Point3<N>, b: Point3<N>) -> Rect<N> {
        Rect { a, b }
    }
}

impl Hittable<f64> for Rect<f64> {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord<f64>) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        let t0 = f64::min(
            (self.a.x - r.orig.x) / r.dir.x,
            (self.b.x - r.orig.x) / r.dir.x,
        );
        let t1 = f64::max(
            (self.a.x - r.orig.x) / r.dir.x,
            (self.b.x - r.orig.x) / r.dir.x,
        );
        t_min = f64::max(t0, t_min);
        t_max = f64::min(t1, t_max);
        if t_max <= t_min {
            return false;
        }

        let t2 = f64::min(
            (self.a.y - r.orig.y) / r.dir.y,
            (self.b.y - r.orig.y) / r.dir.y,
        );
        let t3 = f64::max(
            (self.a.y - r.orig.y) / r.dir.y,
            (self.b.y - r.orig.y) / r.dir.y,
        );
        t_min = f64::max(t2, t_min);
        t_max = f64::min(t3, t_max);
        if t_max <= t_min {
            return false;
        }

        let t4 = f64::min(
            (self.a.z - r.orig.z) / r.dir.z,
            (self.b.z - r.orig.z) / r.dir.z,
        );
        let t5 = f64::max(
            (self.a.z - r.orig.z) / r.dir.z,
            (self.b.z - r.orig.z) / r.dir.z,
        );
        t_min = f64::max(t4, t_min);
        t_max = f64::min(t5, t_max);
        if t_max <= t_min {
            return false;
        }

        rec.t = t_min;
        rec.p = r.at(rec.t);
        rec.normal = rec.p;
        if Vec3::dot(&r.dir, &rec.normal).is_sign_positive() {
            rec.normal = -rec.normal;
        }
        true
    }
}
