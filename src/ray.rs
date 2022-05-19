use crate::vec3::{Point3, Vec3};
use num_traits::Num;

pub struct Ray<N: Num> {
    pub orig: Point3<N>,
    pub dir: Vec3<N>,
}

impl<N: Num + Copy> Ray<N> {
    pub fn new(orig: Vec3<N>, dir: Vec3<N>) -> Ray<N> {
        Ray { orig, dir }
    }

    pub fn at(&self, t: N) -> Point3<N> {
        self.orig + (self.dir * t)
    }
}

impl<N: Num> PartialEq for Ray<N> {
    fn eq(&self, other: &Self) -> bool {
        self.orig == other.orig && self.dir == other.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let origin: Point3<i32> = Point3::new(1, 2, 3);
        let direction: Vec3<i32> = Vec3::new(4, 5, 6);
        let ray: Ray<i32> = Ray::new(origin, direction);
        let res: Point3<i32> = Point3::new(9, 12, 15);
        assert_eq!(ray.at(2), res);
    }
}
