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
    #[allow(unused_variables)]
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord<f64>) -> bool {
        let dirfra = Vec3::new(1.0 / r.dir.x, 1.0 / r.dir.y, 1.0 / r.dir.z);

        let t1 = (self.a.x - r.dir.x) * dirfra.x;
        let t2 = (self.b.x - r.dir.x) * dirfra.x;
        let t3 = (self.a.y - r.dir.y) * dirfra.y;
        let t4 = (self.b.y - r.dir.y) * dirfra.y;
        let t5 = (self.a.z - r.dir.z) * dirfra.z;
        let t6 = (self.b.z - r.dir.z) * dirfra.z;

        let t_min = f64::max(
            f64::max(f64::min(t1, t2), f64::min(t3, t4)),
            f64::min(t5, t6),
        );
        let t_max = f64::min(
            f64::min(f64::max(t1, t2), f64::max(t3, t4)),
            f64::max(t5, t6),
        );

        // if tmax < 0, ray is intersecting AABB, but the whole AABB is behind us
        if t_max < 0.0 {
            return false;
        }

        // if t_max < t_min, ray doesn't intersect AABB
        if t_max < t_min {
            return false;
        }

        rec.t = t_min;
        rec.p = r.at(rec.t);
        rec.normal = -r.dir;
        true
    }
}
