use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use num_traits::{Num, Zero};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vec3<f64>,
    pub t: N,
}

impl<N: Num + Copy> HitRecord {
    pub fn new() -> HitRecord {
        let v = Vec3::new(0.0, 0.0, 0.0);
        HitRecord {
            p: v,
            normal: v,
            t: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList<T: ?Sized + Hittable> {
    pub objects: Vec<Box<T>>,
}

impl<T: ?Sized + Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Box<T>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn size(&self) -> usize {
        self.objects.len()
    }
}

impl<T: ?Sized + Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
