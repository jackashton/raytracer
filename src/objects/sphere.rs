use crate::material::Material;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere<M: Material> {
    center: Point3<f64>,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3<f64>, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length() * ray.dir.length();
        let h = oc.dot(&ray.dir);
        let c = oc.length() * oc.length() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-h - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-h + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        // normal always points against the incident ray
        let mut normal = (point - self.center) / self.radius;
        let mut front_face = true;
        if ray.dir.dot(&normal).is_sign_positive() {
            normal = -normal;
            front_face = false;
        }
        Some(HitRecord {
            t,
            point,
            normal,
            front_face,
            material: &self.material,
        })
    }
}

pub struct MovingSphere<M: Material> {
    center0: Point3<f64>,
    center1: Point3<f64>,
    time0: f64,
    time1: f64,
    radius: f64,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Point3<f64>,
        center1: Point3<f64>,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3<f64> {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.center(ray.time);
        let a = ray.dir.length() * ray.dir.length();
        let h = oc.dot(&ray.dir);
        let c = oc.length() * oc.length() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant.is_sign_negative() {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-h - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-h + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        // normal always points against the incident ray
        let mut normal = (point - self.center(ray.time)) / self.radius;
        let mut front_face = true;
        if ray.dir.dot(&normal).is_sign_positive() {
            normal = -normal;
            front_face = false;
        }
        Some(HitRecord {
            t,
            point,
            normal,
            front_face,
            material: &self.material,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;
    use crate::vec3::Vec3;

    #[test]
    fn test_hit() {
        let center = Point3::new(0.0, 0.0, -1.0);
        let material = Lambertian::new(Vec3::zero());
        let sphere = Sphere::new(center, 0.5, material);
        let origin = Point3::new(0.0, 0.0, 0.0);
        let ray = Ray::new(origin, center, 0.0);
        assert!(sphere.hit(&ray, 0.0, f64::INFINITY).is_some())
    }
}
