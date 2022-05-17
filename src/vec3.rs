use std::ops::{Add, Sub, Mul, Div};
use num_traits::{Num, Float};

#[derive(Debug, Copy, Clone)]
pub struct Vec3<N: Num> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N: Num + Copy> Vec3<N> {
    pub fn new(x: N, y: N, z: N) -> Vec3<N> {
        Vec3 { x, y, z }
    }

    pub fn dot(u: &Vec3<N>, v: &Vec3<N>) -> N {
        (u.x * v.x) + (u.y * v.y) + (u.z * v.z)
    }

    pub fn cross(u: &Vec3<N>, v: &Vec3<N>) -> Vec3<N> {
        Vec3 {
            x: (u.y * v.z) - (u.z * v.y),
            y: (u.z * v.x) - (u.x * v.z),
            z: (u.x * v.y) - (u.y * v.x),
        }
    }
}

impl<N: Float> Vec3<N> {
    pub fn length(&self) -> N {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn unit_vector(v: Vec3<N>) -> Vec3<N> {
        v / v.length()
    }
}

impl<N: Num> Add for Vec3<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<N: Num + Copy> Add<N> for Vec3<N> {
    type Output = Self;

    fn add(self, rhs: N) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl<N: Num> Sub for Vec3<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<N: Num + Copy> Sub<N> for Vec3<N> {
    type Output = Self;

    fn sub(self, rhs: N) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}

impl<N: Num> Mul<Vec3<N>> for Vec3<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl<N: Num + Copy> Mul<N> for Vec3<N> {
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl<N: Num> Div<Vec3<N>> for Vec3<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl<N: Num + Copy> Div<N> for Vec3<N> {
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl<N: Num> PartialEq for Vec3<N> {
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

    let vec1: Vec3<u8> = Vec3::new(1, 2, 3);
    let vec2: Vec3<u8> = Vec3::new(3, 2, 1);
    let res: Vec3<u8> = Vec3::new(4, 4, 4);
    assert_eq!(vec1 + vec2, res);

    let res: Vec3<u8> = Vec3::new(2, 3, 4);
    assert_eq!(vec1 + 1, res)
}

#[test]
fn test_sub() {
    let vec1: Vec3<f64> = Vec3::new(0.2, 0.4, 0.8);
    let vec2: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
    let res: Vec3<f64> = Vec3::new(0.1, 0.2, 0.4);
    assert_eq!(vec1 - vec2, res);

    let vec1: Vec3<u8> = Vec3::new(5, 6, 2);
    let vec2: Vec3<u8> = Vec3::new(3, 2, 1);
    let res: Vec3<u8> = Vec3::new(2, 4, 1);
    assert_eq!(vec1 - vec2, res);

    let res: Vec3<u8> = Vec3::new(4, 5, 1);
    assert_eq!(vec1 - 1, res)
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
fn test_dot() {
    let vec1: Vec3<f64> = Vec3::new(1.0, 3.0, 4.0);
    let vec2: Vec3<f64> = Vec3::new(2.0, 3.0, 5.0);
    let res: f64 = 31.0;
    assert_eq!(Vec3::dot(&vec1, &vec2), res);
}

#[test]
fn test_cross() {
    let vec1: Vec3<i32> = Vec3::new(1, 3, 4);
    let vec2: Vec3<i32> = Vec3::new(2, 3, 5);
    let res: Vec3<i32> = Vec3::new(3, 3, -3);
    assert_eq!(Vec3::cross(&vec1, &vec2), res);
}