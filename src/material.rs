use crate::objects::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::utils::{random_in_unit_sphere, random_unit_vector};
use crate::vec3::Vec3;

fn reflect(v: &Vec3<f64>, n: &Vec3<f64>) -> Vec3<f64> {
    *n * (*v - 2.0) * Vec3::dot(v, n)
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3<f64>)>;
}

pub struct Lambertian {
    albedo: Vec3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vec3<f64>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3<f64>)> {
        let mut target = rec.normal + random_unit_vector();
        // TODO maybe remove this it seems to make little to no difference
        if target.near_zero() {
            target = rec.normal;
        }
        let scattered = Ray::new(rec.p, target);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3<f64>, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3<f64>)> {
        let reflected = reflect(&Vec3::unit_vector(ray_in.dir), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
