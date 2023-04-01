use crate::objects::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::utils::{random_in_unit_sphere, random_unit_vector};
use crate::vec3::Vec3;
use rand::Rng;

fn reflect(v: Vec3<f64>, n: Vec3<f64>) -> Vec3<f64> {
    v - (n * v.dot(&n) * 2.0)
}

fn refract(uv: Vec3<f64>, n: Vec3<f64>, refraction_ratio: f64) -> Vec3<f64> {
    // annoying since min can't be used
    let cos_theta = (-uv).dot(&n).min(1.0);
    let r_out_perp = (uv + (n * cos_theta)) * refraction_ratio;
    let r_out_parallel = n * -(1.0 - (r_out_perp.length().powi(2))).abs().sqrt();
    r_out_perp + r_out_parallel
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3<f64>)>;
}

pub struct Lambertian {
    albedo: Vec3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vec3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3<f64>)> {
        let mut target = hit.normal + random_unit_vector();
        // TODO maybe remove this it seems to make little to no difference
        if target.near_zero() {
            target = hit.normal;
        }
        let scattered = Ray::new(hit.point, target);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Vec3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3<f64>, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3<f64>)> {
        let reflected = reflect(ray_in.dir.normalize(), rec.normal);
        let scattered = Ray::new(rec.point, reflected + random_in_unit_sphere() * self.fuzz);
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_ratio: f64) -> f64 {
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3<f64>)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.dir.normalize();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();

        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rng.gen() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };

        Some((Ray::new(rec.point, direction), Vec3::new(1.0, 1.0, 1.0)))
    }
}
