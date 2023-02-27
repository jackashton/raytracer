use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3<f64>,
    pub normal: Vec3<f64>,
    pub material: &'a dyn Material,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest = t_max;

        for object in &self.objects {
            if let Some(temp) = object.hit(r, t_min, closest) {
                closest = temp.t;
                hit = Some(temp);
            }
        }
        hit
    }
}
