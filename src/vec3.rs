use std::ops::{Add, Sub, Mul, Div};
use num_traits::{Float};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<N: Float> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N: Float> Vec3<N> {
    pub fn new(x: N, y: N, z: N) -> Vec3<N> {
        Vec3 { x, y, z }
    }

    /// Magnitude of the vector
    pub fn length(&self) -> N {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn unit_vector(v: Vec3<N>) -> Vec3<N> {
        v / v.length()
    }

    pub fn dot(v: &Vec3<N>, u: &Vec3<N>) -> N {
        (v.x * u.x) + (v.y * u.y) + (v.z * u.z)
    }
}

impl<N: Float> Add for Vec3<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<N: Float> Add<N> for Vec3<N> {
    type Output = Self;

    fn add(self, rhs: N) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl<N: Float> Sub for Vec3<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<N: Float> Sub<N> for Vec3<N> {
    type Output = Self;

    fn sub(self, rhs: N) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}

impl<N: Float> Mul<Vec3<N>> for Vec3<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl<N: Float> Mul<N> for Vec3<N> {
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl<N: Float> Div<Vec3<N>> for Vec3<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl<N: Float> Div<N> for Vec3<N> {
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl<N: Float> PartialEq for Vec3<N> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[test]
fn test_add() {
    let vec1: Vec3<f64> = Vec3::new(0.2, 0.4, 0.7);
    let vec2: Vec3<f64> = Vec3::new(0.1, 0.3, 0.3);
    let res: Vec3<f64> = Vec3::new(0.30000000000000004, 0.7, 1.0);
    assert_eq!(vec1 + vec2, res);
}

#[test]
fn test_sub() {
    let vec1: Vec3<f64> = Vec3::new(0.2, 0.4, 0.8);
    let vec2: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
    let res: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
    assert_eq!(vec1 - vec2, res);
}

#[test]
fn test_mul() {
    let vec1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    let vec2: Vec3<f64> = Vec3::new(2.0, 2.0, 2.0);
    let res: Vec3<f64> = Vec3::new(2.0, 4.0, 6.0);
    assert_eq!(vec1 * vec2, res);
    assert_eq!(vec1 * 2.0, res);
}

#[test]
fn test_div() {
    let vec1: Vec3<f64> = Vec3::new(1.0, 2.0, 4.0);
    let vec2: Vec3<f64> = Vec3::new(2.0, 2.0, 2.0);
    let res: Vec3<f64> = Vec3::new(0.5, 1.0, 2.0);
    assert_eq!(vec1 / vec2, res);
    assert_eq!(vec1 / 2.0, res);
}

#[test]
fn test_length() {
    let vec1: Vec3<f64> = Vec3::new(0.0, 3.0, 4.0);
    let res: f64 = 5.0;
    assert_eq!(vec1.length(), res);
}

#[test]
fn test_unit_vector() {
    let vec1: Vec3<f64> = Vec3::new(0.0, 3.0, 4.0);
    let res: Vec3<f64> = Vec3::new(0.0, 3.0 / 5.0, 4.0 / 5.0);
    assert_eq!(Vec3::unit_vector(vec1), res);
}

#[test]
fn test_dot_product() {
    let vec1: Vec3<f64> = Vec3::new(1.0, 3.0, 4.0);
    let vec2: Vec3<f64> = Vec3::new(2.0, 3.0, 5.0);
    let res: f64 = 31.0;
    assert_eq!(Vec3::dot(&vec1, &vec2), res);
}