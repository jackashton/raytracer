use crate::objects::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::utils::{random_in_unit_sphere, random_unit_vector};
use crate::vec3::Vec3;

fn reflect(v: &Vec3<f64>, n: &Vec3<f64>) -> Vec3<f64> {
    *n * (*v - 2.0) * Vec3::dot(v, n)
}

fn refract(v: &Vec3<f64>, n: &Vec3<f64>, refraction_ratio: f64) -> Vec3<f64> {
    let uv = Vec3::unit_vector(*v);
    // annoying since min can't be used
    let cos_theta = Vec3::dot(&(-uv), n).min(1.0);
    let r_out_perp = (uv + (*n * cos_theta)) * refraction_ratio;
    let r_out_parallel = *n * -(1.0 - (r_out_perp.length().powi(2))).abs().sqrt();
    r_out_perp + r_out_parallel
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3<f64>)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let refracted = refract(&ray_in.dir, &rec.normal, refraction_ratio);
        Some((Ray::new(rec.p, refracted), Vec3::new(1.0, 1.0, 1.0)))
    }
}
