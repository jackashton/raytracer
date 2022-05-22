use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use num_traits::Num;

pub struct HitRecord<N: Num> {
    pub p: Point3<N>,
    pub normal: Vec3<N>,
    pub t: N,
}

pub trait Hittable<N: Num> {
    fn hit(&self, r: &Ray<N>, t_min: N, t_max: N, rec: &mut HitRecord<N>) -> bool;
}
