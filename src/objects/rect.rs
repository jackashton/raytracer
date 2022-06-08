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

        let inv_d = <[f64; 3]>::from(Vec3::new(1.0 / r.dir.x, 1.0 / r.dir.y, 1.0 / r.dir.z));
        let va = <[f64; 3]>::from(self.a);
        let vb = <[f64; 3]>::from(self.b);
        let o = <[f64; 3]>::from(r.orig);
        let mut t0: f64;
        let mut t1: f64;

        for i in 0..3 {
            t0 = (va[i] - o[i]) * inv_d[i];
            t1 = (vb[i] - o[i]) * inv_d[i];
            if inv_d[i] < 0.0 {
                (t0, t1) = (t1, t0)
            }
            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit() {
        let center = Point3::new(0.0, 0.0, -1.0);
        let rect = Rect::new(
            center - Point3::new(0.5, 0.5, 0.1),
            center + Point3::new(0.5, 0.5, 0.0),
        );
        let origin = Point3::new(0.0, 0.0, 0.0);
        let ray = Ray::new(origin, center);
        let mut rec: HitRecord<f64> = HitRecord::new();
        assert!(rect.hit(&ray, 0.0, f64::INFINITY, &mut rec))
    }
}
