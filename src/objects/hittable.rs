use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use num_traits::{Num, Zero};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord<N: Num> {
    pub p: Point3<N>,
    pub normal: Vec3<N>,
    pub t: N,
}

impl<N: Num + Copy> HitRecord<N> {
    pub fn new() -> HitRecord<N> {
        let zero = Zero::zero();
        let v = Vec3::new(zero, zero, zero);
        HitRecord {
            p: v,
            normal: v,
            t: zero,
        }
    }
}

pub trait Hittable<N: Num> {
    fn hit(&self, r: &Ray<N>, t_min: N, t_max: N, rec: &mut HitRecord<N>) -> bool;
}

pub struct HittableList<T: Hittable<f64>> {
    pub objects: Vec<T>,
}

impl<T: Hittable<f64>> HittableList<T> {
    pub fn new() -> HittableList<T> {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }
}

impl<T: Hittable<f64>> Hittable<f64> for HittableList<T> {
    fn hit(&self, r: &Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord<f64>) -> bool {
        let mut temp = HitRecord::new();
        let mut hit = false;
        let mut closest = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest, &mut temp) {
                hit = true;
                closest = temp.t;
                // TODO fix this so we can do rec = temp or similar
                rec.p = temp.p;
                rec.t = temp.t;
                rec.normal = temp.normal;
            }
        }
        hit
    }
}
