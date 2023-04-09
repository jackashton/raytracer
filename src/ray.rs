use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3<f64>,
    pub dir: Vec3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(orig: Vec3<f64>, dir: Vec3<f64>, time: f64) -> Self {
        Self { orig, dir, time }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.orig + (self.dir * t)
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.orig == other.orig && self.dir == other.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray: Ray = Ray::new(origin, direction, 0.0);
        let res = Point3::new(9.0, 12.0, 15.0);
        assert_eq!(ray.at(2.0), res);
    }
}
